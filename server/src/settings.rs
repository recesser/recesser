use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Settings {
    addr: String,
    database_addr: String,
    objectstorage_addr: String,
}

impl Settings {
    pub fn new() -> Self {
        let mut settings = Config::new();
        settings.merge(File::with_name("config.toml").required(false));
        settings.merge(Environment::with_prefx("recesser"));

        settings.into()
    }
}
