use dotenvy::dotenv;
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_value::Value;
use std::sync::RwLock;

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
    /// Used directly by sqlx
    pub database_url: String,

    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    pub level: String,

    #[serde(default = "default_log_format")]
    pub format: String,

    #[serde(default = "default_log_output")]
    pub output: String,
}

/* ---------------- defaults ---------------- */

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

pub fn load_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    dotenv().ok();

    let app = envy::prefixed("APP_APP__").from_env::<AppInfo>()?;
    let server = envy::prefixed("APP_SERVER__").from_env::<ServerConfig>()?;
    let log = envy::prefixed("APP_LOG__").from_env::<LogConfig>()?;

    // 👇 DATABASE_URL is global, no prefix
    let database_url = std::env::var("DATABASE_URL")?;

    let database = DatabaseConfig {
        database_url,
        max_connections: envy::prefixed("APP_DATABASE__")
            .from_env::<DatabaseConfig>()
            .unwrap_or(DatabaseConfig {
                database_url: String::new(), // overwritten
                max_connections: default_max_connections(),
            })
            .max_connections,
    };

    Ok(AppConfig {
        app,
        server,
        database,
        log,
    })
}

pub static CONFIG: Lazy<RwLock<AppConfig>> = Lazy::new(|| {
    let config = load_config().expect("Failed to load configuration");
    RwLock::new(config)
});

pub fn get_config() -> AppConfig {
    CONFIG.read().unwrap().clone()
}

pub fn get<T: DeserializeOwned>(key: &str) -> Option<T> {
    let config = get_config();
    let value = serde_value::to_value(&config).ok()?;

    let mut current = &value;
    for part in key.split('.') {
        match current {
            Value::Map(map) => {
                current = map.get(&Value::String(part.to_string()))?;
            }
            _ => return None,
        }
    }

    T::deserialize(current.clone()).ok()
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

pub fn init() {
    let _unused = CONFIG.read().unwrap();
}

/* ---------------- tests ---------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        let config = load_config();
        assert!(config.is_ok());
    }

    #[test]
    fn test_get_string() {
        init();
        let app_name = get_string("app.name");
        assert!(app_name.is_some());
    }
}
