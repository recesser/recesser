use std::path::{Path, PathBuf};

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub addr: String,
}

impl Settings {
    pub fn new(filepath: &Option<PathBuf>) -> std::result::Result<Self, ConfigError> {
        let mut settings = Config::new();

        settings.set_default("addr", "http://recesser-server.recesser")?;

        let filepath = match filepath {
            Some(path) => path,
            None => Path::new("recesser.toml"),
        };
        settings.merge(File::from(filepath).required(false))?;
        settings.merge(Environment::with_prefix("recesser"))?;

        settings.try_into()
    }
}
