use bollard::Docker;

#[derive(Clone)]
pub struct AppState {
    pub docker: Docker,
}
