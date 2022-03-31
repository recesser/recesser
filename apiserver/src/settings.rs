use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    pub addr: String,
    pub objectstorage_addr: String,
    pub database_addr: String,
    pub secretstorage_addr: String,
    pub log_level: String,
}

impl Settings {
    pub fn new() -> std::result::Result<Self, ConfigError> {
        let config = Config::builder()
            .set_default("addr", "0.0.0.0:8080")?
            .set_default("objectstorage_addr", "http://minio.minio:9000")?
            .set_default("database_addr", "redis://redis.redis/")?
            .set_default("secretstorage_addr", "http://vault.vault:8200")?
            .set_default("log_level", "info")?
            .add_source(File::with_name("config.toml").required(false))
            .add_source(Environment::with_prefix("recesser"))
            .build()?;
        config.try_deserialize()
    }
}
