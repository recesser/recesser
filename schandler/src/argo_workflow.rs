use std::convert::TryFrom;

use anyhow::{Error, Result};
use serde::Serialize;
use std::env;

use crate::workflow::Workflow;

#[derive(Serialize, Debug)]
pub struct ArgoWorkflow {}

impl ArgoWorkflow {
    pub async fn enqueue(&self, argo_addr: &str) -> Result<()> {
        let client = reqwest::Client::new();
        client
            .post(format!("http://{argo_addr}/api/v1/workflows/argo/submit"))
            .bearer_auth(env::var("BEARER_TOKEN")?)
            .json(self)
            .send()
            .await?;
        Ok(())
    }
}

impl TryFrom<Workflow> for ArgoWorkflow {
    type Error = Error;

    fn try_from(value: Workflow) -> Result<Self, Self::Error> {
        Ok(ArgoWorkflow {})
    }
}
