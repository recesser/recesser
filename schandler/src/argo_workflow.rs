use std::convert::TryFrom;

use anyhow::{Error, Result};
use reqwest::Client;
use serde::Serialize;

use crate::workflow::Workflow;

#[derive(Clone)]
pub struct ArgoWorkflowServer {
    addr: String,
    client: Client,
}

impl ArgoWorkflowServer {
    pub fn new(addr: &str) -> Self {
        Self {
            addr: String::from(addr),
            client: reqwest::Client::new(),
        }
    }

    pub async fn submit(&self, argo_workflow: ArgoWorkflow) -> Result<()> {
        self.client
            .post(format!("http://{}/api/v1/workflows/argo/submit", self.addr))
            .json(&argo_workflow)
            .send()
            .await?;
        Ok(())
    }
}

#[derive(Serialize, Debug)]
pub struct ArgoWorkflow {}

impl TryFrom<Workflow> for ArgoWorkflow {
    type Error = Error;

    fn try_from(value: Workflow) -> Result<Self, Self::Error> {
        Ok(ArgoWorkflow {})
    }
}
