mod template;

use std::fs;

use anyhow::Result;
use recesser_core::encoding::hex;
use recesser_core::repository::Repository;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};

use crate::workflow::{Kind, Workflow};

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
        let cb = Client::builder()
            .default_headers(headers)
            .danger_accept_invalid_certs(true);
        Ok(Self {
            addr: String::from(addr),
            client: cb.build()?,
        })
    }

    pub async fn submit(&self, workflow: &ArgoWorkflow) -> Result<()> {
        tracing::debug!(message = "Submitting workflow", workflow = ?workflow);
        let result = self
            .client
            .post(format!("{}/api/v1/workflows/argo", self.addr))
            .json(&serde_json::json!({ "namespace": "argo", "serverDryRun": false, "workflow": workflow }))
            .send()
            .await?;
        tracing::debug!(message = "Result from argo", result = ?result);
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(transparent)]
pub struct ArgoWorkflow(serde_json::Value);

impl ArgoWorkflow {
    pub fn from_workflow(workflow: Workflow, repository: Repository) -> Result<Self> {
        let metadata = workflow.metadata;
        let workflow = match workflow.kind {
            Kind::TemplateWorkflow(workflow) => construct_from_template(
                Template::TemplateWorkflow,
                minijinja::context!(
                    metadata,
                    workflow,
                    repository => minijinja::context!(
                        name => repository.name,
                        url => repository.url,
                        ssh_key_fingerprint => hex::encode_str(&repository.public_key.fingerprint.to_string())?
                    )
                ),
            )?,
            _ => return Err(anyhow::anyhow!("CustomTemplate is not yet implemented")),
        };

        Ok(workflow)
    }
}
