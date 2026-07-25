mod docker;
mod routes;
mod state;

use axum::{Router, routing::get};
use bollard::Docker;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::state::AppState;

#[tokio::main]
async fn main() {
    // tracing subscriber
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // connect to the docker daemon once, at startup
    let docker =
        Docker::connect_with_local_defaults().expect("failed to connect to the Docker daemon");
    // add it to the shared app state
    let state = AppState { docker };
    // making the app
    let app = Router::new()
        .route("/", get(routes::root::root))
        .route("/up", get(routes::up::health_check))
        .route("/version", get(routes::version::get_version))
        .layer(
            ServiceBuilder::new().layer(
                TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
            ),
        )
        .with_state(state);

    //serving
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
