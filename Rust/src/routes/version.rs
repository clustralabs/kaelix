use axum::{extract::State, http::StatusCode, response::Json};
use serde_json::{Value, json};

use crate::state::AppState;

#[tracing::instrument(skip(state))]
pub async fn get_version(State(state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    tracing::info!("querying docker daemon version");

    let version = state.docker.version().await.map_err(|err| {
        tracing::error!(?err, "failed to reach the docker daemon");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tracing::info!(
        version = version.version.as_deref().unwrap_or("unknown"),
        "got docker version"
    );

    Ok(Json(json!({
        "version": version.version,
        "api_version": version.api_version,
        "os": version.os,
        "arch": version.arch,
    })))
}
