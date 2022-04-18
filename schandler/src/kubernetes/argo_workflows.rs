use anyhow::Result;
use recesser_core::repository::Repository;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::kubernetes::template::{construct_from_template, Template};
use crate::pipeline::{Kind, Pipeline};

/// HTTP client for Argo Workflows server
#[derive(Clone)]
pub struct ArgoWorkflowsServer {
    addr: String,
    client: Client,
}

impl ArgoWorkflowsServer {
    pub fn new(addr: &str) -> Self {
        Self {
            addr: String::from(addr),
            client: reqwest::Client::new(),
        }
    }

    pub async fn submit(&self, workflow: &Workflow) -> Result<()> {
        self.client
            .post(format!("http://{}/api/v1/workflows/argo/submit", self.addr))
            .json(workflow)
            .send()
            .await?;
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(transparent)]
pub struct Workflow(serde_json::Value);

impl Workflow {
    pub fn from_pipeline(pipeline: Pipeline, repository: Repository) -> Result<Self> {
        let metadata = pipeline.metadata;
        let workflow = match pipeline.kind {
            Kind::TemplatePipeline(pipeline) => construct_from_template(
                Template::TemplateWorkflow,
                minijinja::context!(metadata, pipeline, repository),
            )?,
            _ => return Err(anyhow::anyhow!("CustomTemplate is not yet implemented")),
        };

        Ok(workflow)
    }
}
