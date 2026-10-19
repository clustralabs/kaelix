use bollard::Docker;
use turso::Connection;

#[derive(Clone)]
pub struct AppState {
    pub docker: Docker,
    pub db: Connection, // Connection is cheaply cloneable (internally Arc handles)
}
