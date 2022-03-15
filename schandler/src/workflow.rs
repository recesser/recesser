use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use tokio::fs;

use crate::repository::Repository;

#[derive(Deserialize, Serialize, Debug)]
pub struct Workflow {
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

impl Workflow {
    pub async fn from_repo(repo: &Repository) -> Result<Self> {
        let buf = fs::read_to_string(repo.join("recesser.yaml")).await?;
        Ok(serde_yaml::from_str(&buf)?)
    }
}
