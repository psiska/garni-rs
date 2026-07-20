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
use clap::{Arg, Command, crate_authors, crate_description, crate_version};
use server::handler::{prometheus_list_v004_handler, weather_set_handler};
use std::path::PathBuf;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    let cmd = Command::new("garni-rs")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("config-file")
                .long("config-file")
                .value_name("FILE")
                .help("Path to the configuration file")
                .default_value("/etc/garnirs/config.toml") // Sets the default value & implicitly makes it optional
                .value_parser(clap::value_parser!(PathBuf)),
        );

    let matches = cmd.get_matches();

    let config_file = matches
        .get_one::<PathBuf>("config-file")
        .expect("A default value is guaranteed");

    // Check if the file exists and is actually a file (not a directory)
    if !config_file.is_file() {
        error!(name: "startup", "garni-rs:server: Configuration file {:?} does not exist.",
            config_file
        );
    }

    tracing_subscriber::fmt()
        // This allows you to use, e.g., `RUST_LOG=info` or `RUST_LOG=debug`
        // when running the app to set log levels.
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("garni_rs=info,tower_http=debug"))
                .unwrap(),
        )
        .init();

    let config = load_config(config_file.clone()).unwrap_or_else(|err| {
        error!(name: "startup", "garni-rs:server: Configuration error: {}", err);
        std::process::exit(1);
    });
    info!(name: "startup", "garni-rs:server: starting");
    info!(name: "startup", "garni-rs:server: config value: server.host = {}", config.server.host);
    info!(name: "startup", "garni-rs:server: config value: server.port = {}", config.server.port);
    info!(name: "startup", "garni-rs:server: config value: server.garni_update_path = {}", config.server.garni_update_path);
    info!(name: "startup", "garni-rs:server: config value: server.prometheus_v004_path = {}", config.server.prometheus_v004_path);
    info!(name: "startup", "garni-rs:server: config value: server.prometheus_v1_path = {}", config.server.prometheus_v1_path);
    info!(name: "startup", "garni-rs:server: config value: server.prometheus_openmetrics_path = {}", config.server.prometheus_openmetrics_path);
    info!(name: "startup", "garni-rs:server: config value: logging.level = {}", config.logging.level);

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
