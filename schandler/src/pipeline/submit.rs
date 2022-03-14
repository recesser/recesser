use anyhow::Result;
use serde::Serialize;
use std::env;

pub async fn submit(transformed_pipeline: &impl Serialize, argo_addr: &str) -> Result<()> {
    let client = reqwest::Client::new();
    client
        .post(format!("http://{argo_addr}/api/v1/workflows/argo/submit"))
        .bearer_auth(env::var("BEARER_TOKEN")?)
        .json(transformed_pipeline)
        .send()
        .await?;
    Ok(())
}
