use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub addr: String,
}

impl Settings {
    pub fn new() -> std::result::Result<Self, ConfigError> {
        let mut settings = Config::new();

        settings.set_default("addr", "http://recesser-server.recesser")?;

        settings.merge(File::with_name("config.toml").required(false))?;
        settings.merge(Environment::with_prefix("recesser"))?;

        settings.try_into()
    }
}
