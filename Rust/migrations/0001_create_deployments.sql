-- kaelix deployments: one row per container kaelix manages.
-- Columns mirror create_container()/pull_image() in src/docker.rs.
CREATE TABLE IF NOT EXISTS deployments (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    name       TEXT NOT NULL UNIQUE,              -- docker container name
    image      TEXT NOT NULL,                     -- e.g. 'nginx'
    tag        TEXT NOT NULL DEFAULT 'latest',    -- pulled via pull_image()
    domain     TEXT NOT NULL UNIQUE,              -- traefik Host() rule
    port       INTEGER NOT NULL CHECK (port BETWEEN 1 AND 65535),
    status     TEXT NOT NULL DEFAULT 'created'
               CHECK (status IN ('created', 'starting', 'running', 'stopped', 'failed')),
    created_at TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP),
    updated_at TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP)
);
