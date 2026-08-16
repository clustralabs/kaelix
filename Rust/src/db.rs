use std::time::Duration;

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

/// Build the connection pool. `connect_lazy` only parses the URL here
/// (so a malformed DATABASE_URL panics immediately) and opens real
/// connections on first use — an unreachable Postgres won't block startup.
pub fn connect(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(2))
        .connect_lazy(database_url)
        .expect("invalid DATABASE_URL")
}

/// Run embedded migrations at startup. Failures are logged, not fatal:
/// the pool recovers once Postgres is reachable and /up reports 503 meanwhile.
pub async fn run_migrations(pool: &PgPool) {
    match sqlx::migrate!("./migrations").run(pool).await {
        Ok(_) => tracing::info!("database migrations applied"),
        Err(err) => tracing::error!(%err, "database migrations failed to apply"),
    }
}
