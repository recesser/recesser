use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub apiserver_addr: String,
    pub argo_workflows_addr: String,
    pub polling_interval: u64,
    pub log_level: String,
}

impl Settings {
    pub fn new() -> std::result::Result<Self, ConfigError> {
        let config = Config::builder()
            .set_default("apiserver_addr", "http://apiserver.recesser:8080")?
            .set_default(
                "argo_workflows_addr",
                "http://argo_workflows.argo_workflows:8080",
            )?
            .set_default("polling_interval", 5)?
            .set_default("log_level", "info")?
            .add_source(Environment::with_prefix("recesser"))
            .build()?;

        config.try_deserialize()
    }
}
