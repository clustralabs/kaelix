use bollard::Docker;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub docker: Docker,
    pub pool: PgPool, // PgPool is cheaply cloneable (internally an Arc)
}
