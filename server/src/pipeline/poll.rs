use anyhow::Result;
use tempfile::tempdir;

use super::git;
use super::submit::submit;

pub async fn poll(repository: &str) -> Result<()> {
    let dir = tempdir()?;
    let dirpath = dir.path();

    git::clone(repository, dirpath)?;

    // Parse and transform pipeline
    let transformed_pipeline = serde_json::json!("Hans");

    submit(&transformed_pipeline).await?;
    Ok(())
}
