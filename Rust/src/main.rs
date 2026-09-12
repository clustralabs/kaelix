mod db;
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

    // Postgres: URL from env, defaulting to the dev compose setup
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://kaelix:kaelix@localhost:5432/kaelix".to_string());
    let pool = db::connect(&database_url);
    db::run_migrations(&pool).await;

    // add it to the shared app state
    let state = AppState { docker, pool };
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
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
