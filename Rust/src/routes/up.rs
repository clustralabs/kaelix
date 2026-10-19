use axum::{extract::State, http::StatusCode};

use crate::state::AppState;

#[tracing::instrument(skip(state))]
pub async fn health_check(State(state): State<AppState>) -> StatusCode {
    match state.db.query("SELECT 1", ()).await {
        Ok(mut rows) => match rows.next().await {
            Ok(_) => {
                tracing::debug!("database ping ok");
                StatusCode::OK
            }
            Err(err) => {
                tracing::error!(%err, "database ping failed reading row");
                StatusCode::SERVICE_UNAVAILABLE
            }
        },
        Err(err) => {
            tracing::error!(%err, "database ping failed");
            StatusCode::SERVICE_UNAVAILABLE
        }
    }
}
