use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub addr: String,
    pub database_addr: String,
    pub objectstorage_addr: String,
    pub log_level: String,
}

impl Settings {
    pub fn new() -> std::result::Result<Self, ConfigError> {
        let mut settings = Config::new();

        settings.set_default("addr", "0.0.0.0:8080")?;
        settings.set_default("database_addr", "redis://redis.redis/")?;
        settings.set_default("objectstorage_addr", "http://minio.minio:9000")?;
        settings.set_default("log_level", "info")?;

        settings.merge(File::with_name("config.toml").required(false))?;
        settings.merge(Environment::with_prefix("recesser"))?;

        settings.try_into()
    }
}
