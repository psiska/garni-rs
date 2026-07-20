mod domain;
mod server;

use crate::domain::model;
use crate::server::config::{Settings, load_config};
use axum::Router;
use axum::http::{
    HeaderValue, Method,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
};
use axum::routing::get;
use server::handler::{prometheus_list_v004_handler, weather_set_handler};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        // This allows you to use, e.g., `RUST_LOG=info` or `RUST_LOG=debug`
        // when running the app to set log levels.
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| {
                    EnvFilter::try_new("garni_rs=info,axum_tracing_example=info,tower_http=info")
                })
                .unwrap(),
        )
        .init();

    let config = load_config().unwrap_or_else(|err| {
        eprintln!("Configuration error: {}", err);
        std::process::exit(1);
    });

    let bind_address = format!("{}:{}", config.server.host, config.server.port);
    let cors_address = format!("http://{}:{}", config.server.host, config.server.port);

    let cors = CorsLayer::new()
        .allow_origin(cors_address.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(&config)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    info!("garni-rs:server: started");

    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub fn create_router(config: &Settings) -> Router {
    let app_state = model::create_app_state();

    Router::new()
        // .route("/api/healthchecker", get(health_checker_handler))
        .route(
            config.server.garni_update_path.as_str(),
            get(weather_set_handler),
        )
        .route(
            config.server.prometheus_v004_path.as_str(),
            get(prometheus_list_v004_handler),
        )
        .with_state(app_state)
}
