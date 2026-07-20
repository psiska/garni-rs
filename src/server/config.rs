use config::{Config, Environment, File};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerConfig,
    pub logging: LoggingConfig,
}

fn default_port() -> u16 {
    8305
}

fn default_host() -> String {
    "0.0.0.0".into()
}

fn default_log_level() -> String {
    "info".into()
}

fn default_garni_update_path() -> String {
    "/weatherstation/updateweatherstation.php".into()
}

fn default_prometheus_v004_path() -> String {
    "/metrics/prometheus".into()
}
fn default_prometheus_v1_path() -> String {
    "/metrics/prometheus_v1".into()
}

fn default_prometheus_openmetrics_path() -> String {
    "/metrics/openmetrics".into()
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_garni_update_path")]
    pub garni_update_path: String,
    #[serde(default = "default_prometheus_v004_path")]
    pub prometheus_v004_path: String,
    #[serde(default = "default_prometheus_v1_path")]
    pub prometheus_v1_path: String,
    #[serde(default = "default_prometheus_openmetrics_path")]
    pub prometheus_openmetrics_path: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    #[serde(default = "default_log_level")]
    pub level: String,

    #[serde(default)] // Uses String::default(), which is empty string
    pub format: String,
}

pub fn load_config(pb: PathBuf) -> Result<Settings, config::ConfigError> {
    let settings = Config::builder()
        .add_source(File::with_name("config.toml").required(false))
        .add_source(File::from(pb).required(false))
        // Environment variables take highest precedence
        // APP_SERVER__PORT=9000 would override server.port
        .add_source(
            Environment::with_prefix("GARNIRS")
                .separator("__")
                .try_parsing(true),
        )
        .build()?;

    settings.try_deserialize()
}
