use anyhow::Result;
use serde::Serialize;

pub async fn submit(transformed_pipeline: &impl Serialize) -> Result<()> {
    let client = reqwest::Client::new();
    let argo_addr = "";
    client
        .post(format!("http://{argo_addr}/api/v1/workflows/argo/submit"))
        .bearer_auth("")
        .json(transformed_pipeline)
        .send()
        .await?;
    Ok(())
}
