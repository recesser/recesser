mod template;

use std::fs;

use anyhow::Result;
use recesser_core::repository::Repository;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};

use crate::pipeline::{Kind, Pipeline};

use template::{construct_from_template, Template};

const TOKEN_PATH: &str = "/var/run/secrets/kubernetes.io/serviceaccount/token";

/// HTTP client for Argo Workflows server
#[derive(Clone)]
pub struct ArgoWorkflowsServer {
    addr: String,
    client: Client,
}

impl ArgoWorkflowsServer {
    pub fn new(addr: &str) -> Result<Self> {
        let token = fs::read_to_string(TOKEN_PATH)?;
        let mut headers = header::HeaderMap::new();
        headers.insert(header::AUTHORIZATION, format!("Bearer {token}").try_into()?);
        let cb = Client::builder().default_headers(headers);
        Ok(Self {
            addr: String::from(addr),
            client: cb.build()?,
        })
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
