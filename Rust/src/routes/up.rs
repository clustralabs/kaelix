use axum::{extract::State, http::StatusCode};

use crate::state::AppState;

#[tracing::instrument(skip(state))]
pub async fn health_check(State(state): State<AppState>) -> StatusCode {
    match sqlx::query("SELECT 1").execute(&state.pool).await {
        Ok(_) => {
            tracing::debug!("database ping ok");
            StatusCode::OK
        }
        Err(err) => {
            tracing::error!(%err, "database ping failed");
            StatusCode::SERVICE_UNAVAILABLE
        }
    }
}
