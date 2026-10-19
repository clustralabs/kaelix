use turso::Connection;

/// Build the Turso database connection.
/// Supports both local SQLite/libSQL files and remote Turso Cloud databases with synchronization.
pub async fn connect() -> Result<Connection, turso::Error> {
    let raw_url = std::env::var("TURSO_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .unwrap_or_default();
    let auth_token = std::env::var("TURSO_AUTH_TOKEN").ok();

    if raw_url.starts_with("libsql://")
        || raw_url.starts_with("https://")
        || raw_url.starts_with("http://")
        || raw_url.starts_with("turso://")
    {
        tracing::info!(url = %raw_url, "connecting to remote Turso database");
        let local_path =
            std::env::var("TURSO_LOCAL_PATH").unwrap_or_else(|_| "kaelix.db".to_string());
        let mut builder = turso::sync::Builder::new_remote(&local_path).with_remote_url(&raw_url);
        if let Some(token) = auth_token {
            builder = builder.with_auth_token(token);
        }
        let db = builder.build().await?;
        db.connect().await
    } else {
        let path = if raw_url.is_empty() {
            "kaelix.db"
        } else if raw_url.starts_with("postgres://") {
            tracing::warn!("detected legacy postgres DATABASE_URL, defaulting to local kaelix.db for Turso");
            "kaelix.db"
        } else {
            raw_url.strip_prefix("file:").unwrap_or(&raw_url)
        };
        tracing::info!(path, "connecting to local Turso / libSQL database");
        let db = turso::Builder::new_local(path).build().await?;
        db.connect()
    }
}

/// Run embedded migrations at startup.
pub async fn run_migrations(conn: &Connection) {
    const MIGRATIONS: &[(&str, &str)] = &[(
        "0001_create_deployments",
        include_str!("../migrations/0001_create_deployments.sql"),
    )];

    let init_migrations_table = "CREATE TABLE IF NOT EXISTS _migrations (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE,
        applied_at TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP)
    );";

    if let Err(err) = conn.execute(init_migrations_table, ()).await {
        tracing::error!(%err, "failed to initialize _migrations table");
        return;
    }

    for (name, sql) in MIGRATIONS {
        let check_sql = "SELECT 1 FROM _migrations WHERE name = ?1 LIMIT 1";
        match conn.prepare(check_sql).await {
            Ok(mut stmt) => match stmt.query([*name]).await {
                Ok(mut rows) => {
                    let already_applied = match rows.next().await {
                        Ok(Some(_)) => true,
                        Ok(None) => false,
                        Err(err) => {
                            tracing::error!(%err, migration = %name, "failed to check migration status");
                            continue;
                        }
                    };
                    if already_applied {
                        tracing::debug!(migration = %name, "migration already applied");
                        continue;
                    }
                }
                Err(err) => {
                    tracing::error!(%err, migration = %name, "failed to query migration status");
                    continue;
                }
            },
            Err(err) => {
                tracing::error!(%err, migration = %name, "failed to prepare migration check statement");
                continue;
            }
        }

        tracing::info!(migration = %name, "applying migration");
        if let Err(err) = conn.execute_batch(sql).await {
            tracing::error!(%err, migration = %name, "failed to execute migration");
            return;
        }

        let record_sql = "INSERT INTO _migrations (name) VALUES (?1)";
        if let Err(err) = conn.execute(record_sql, [*name]).await {
            tracing::error!(%err, migration = %name, "failed to record migration in _migrations");
            return;
        }

        tracing::info!(migration = %name, "migration applied successfully");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_turso_local_connection_and_migrations() {
        let db = turso::Builder::new_local(":memory:")
            .build()
            .await
            .expect("failed to build in-memory db");
        let conn = db.connect().expect("failed to connect");

        run_migrations(&conn).await;

        // Verify migrations table has 0001_create_deployments
        let mut stmt = conn
            .prepare("SELECT name FROM _migrations WHERE name = ?1")
            .await
            .unwrap();
        let mut rows = stmt.query(["0001_create_deployments"]).await.unwrap();
        assert!(rows.next().await.unwrap().is_some());

        // Verify deployments table was created and supports insert/select
        conn.execute(
            "INSERT INTO deployments (name, image, domain, port) VALUES (?1, ?2, ?3, ?4)",
            ("web-test", "nginx", "test.local", 80),
        )
        .await
        .unwrap();

        let mut stmt = conn
            .prepare("SELECT name, status, port FROM deployments WHERE name = ?1")
            .await
            .unwrap();
        let mut rows = stmt.query(["web-test"]).await.unwrap();
        let row = rows.next().await.unwrap().expect("row expected");
        assert_eq!(row.get_value(0).unwrap(), turso::Value::Text("web-test".into()));
        assert_eq!(row.get_value(1).unwrap(), turso::Value::Text("created".into()));
        assert_eq!(row.get_value(2).unwrap(), turso::Value::Integer(80));

        // Test running migrations again (should be idempotent)
        run_migrations(&conn).await;

        // Verify health check ping query works
        let mut ping_rows = conn.query("SELECT 1", ()).await.unwrap();
        assert!(ping_rows.next().await.unwrap().is_some());
    }
}
