use serde::Deserialize;
use config::{Config, ConfigError, File};

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub defaults: DefaultParams,
    pub engine: EngineConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EngineConfig {
    pub device: String, // "cpu" or "accelerated:0"
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub path: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DefaultParams {
    pub seed: u32,
    pub steps: u32,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("Settings").required(false))
            .add_source(config::Environment::with_prefix("NEBULA"))
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 3001)?
            .set_default("database.path", "nebula_canvas.redb")?
            .set_default("defaults.seed", 42)?
            .set_default("defaults.steps", 20)?
            .set_default("engine.device", "cpu")?
            .build()?;

        s.try_deserialize()
    }
}
