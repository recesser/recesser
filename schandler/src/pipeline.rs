use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::repository::LocalRepository;

#[derive(Deserialize, Serialize, Debug)]
pub struct Pipeline {
    api_version: String,
    metadata: Metadata,
    #[serde(flatten)]
    kind: Kind,
}

#[derive(Deserialize, Serialize, Debug)]
struct Metadata {
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "kind", content = "spec")]
enum Kind {
    TemplatePipeline {
        inputs: Vec<String>,
        template: Template,
        dependencies: Vec<String>,
        command: Vec<String>,
        args: Vec<String>,
        working_dir: String,
    },
    CustomPipeline {
        inputs: Vec<String>,
        image: String,
        build: Option<String>,
        command: Vec<String>,
        args: Vec<String>,
        working_dir: String,
    },
}

#[derive(Deserialize, Serialize, Debug)]
struct Template {
    name: String,
    version: String,
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
