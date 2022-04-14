use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::repository::LocalRepository;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Pipeline {
    pub api_version: String,
    pub metadata: Metadata,
    #[serde(flatten)]
    pub kind: Kind,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Metadata {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "kind", content = "spec")]
pub enum Kind {
    TemplatePipeline(TemplatePipeline),
    CustomPipeline(CustomPipeline),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TemplatePipeline {
    pub inputs: Option<Vec<String>>,
    pub template: Template,
    pub dependencies: Option<String>,
    pub entrypoint: Vec<String>,
    pub args: Option<Vec<String>>,
    pub working_dir: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Template {
    pub name: String,
    pub version: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomPipeline {
    pub inputs: Vec<String>,
    pub image: Option<String>,
    pub build: Option<String>,
    pub entrypoint: Vec<String>,
    pub args: Option<Vec<String>>,
    pub working_dir: Option<String>,
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
