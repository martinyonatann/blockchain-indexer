use std::sync::RwLock;

use config::{Config, ConfigError, Environment, File};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub app: AppInfo,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub log: LogConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub environment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    #[serde(default = "default_workers")]
    pub workers: usize,
    #[serde(default)]
    pub timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
    #[serde(default)]
    pub ssl_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    pub level: String,
    #[serde(default = "default_log_format")]
    pub format: String,
    #[serde(default = "default_log_output")]
    pub output: String,
}

fn default_max_connections() -> u32 {
    10
}
fn default_workers() -> usize {
    4
}
fn default_log_format() -> String {
    "json".to_string()
}
fn default_log_output() -> String {
    "stdout".to_string()
}

pub fn load_config() -> Result<AppConfig, ConfigError> {
    let config = Config::builder()
        .add_source(File::with_name("config/config.yaml").required(true))
        .add_source(
            Environment::with_prefix("APP")
                .separator("__")
                .try_parsing(true),
        )
        .build()?;

    config.try_deserialize::<AppConfig>()
}

pub static CONFIG: Lazy<RwLock<AppConfig>> =
    Lazy::new(|| RwLock::new(load_config().expect("Failed to load configuration")));

pub fn get_config() -> AppConfig {
    CONFIG.read().unwrap().clone()
}

pub fn get<T: serde::de::DeserializeOwned>(key: &str) -> Option<T> {
    let config = get_config();
    let value = serde_yaml::to_value(&config).ok()?;

    // Navigate nested keys (e.g., "app.name")
    let mut current = &value;
    for part in key.split('.') {
        current = current.get(part)?;
    }

    serde_yaml::from_value(current.clone()).ok()
}

pub fn get_string(key: &str) -> Option<String> {
    get::<String>(key)
}

pub fn get_int(key: &str) -> Option<i64> {
    get::<i64>(key)
}

pub fn get_bool(key: &str) -> Option<bool> {
    get::<bool>(key)
}

pub fn init() -> Result<(), ConfigError> {
    let _unused = CONFIG.read().unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        println!("cwd: {:?}", std::env::current_dir().unwrap());
        let config = load_config();

        assert!(config.is_ok())
    }

    #[test]
    fn test_get_string() {
        let _unused = CONFIG.read().unwrap();
        let app_name = get_string("app.name");

        assert!(app_name.is_some());
    }
}
