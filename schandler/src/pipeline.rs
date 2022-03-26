use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use tokio::fs;

use crate::repository::LocalRepository;

#[derive(Deserialize, Serialize, Debug)]
pub struct Pipeline {
    name: String,
    artifact: String,
    template: Option<Template>,
    custom_workflow: Option<Value>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Template {
    language: Language,
    image: Option<String>,
    entrypoint: PathBuf,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum Language {
    Python,
    R,
}

impl Pipeline {
    pub async fn from_repo(repo: &LocalRepository) -> Result<Self> {
        let workflow_path = repo.path.join("recesser.yaml");
        if !workflow_path.exists() {
            anyhow::bail!("Repository doesn't contain a pipeline definition (recesser.yaml).");
        }
        let buf = fs::read_to_string(&workflow_path).await?;
        Ok(serde_yaml::from_str(&buf)?)
    }
}
