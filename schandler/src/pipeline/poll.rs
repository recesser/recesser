use std::path::Path;

use anyhow::Result;
use tempfile::tempdir;
use tokio::fs;
use tokio::time;

use super::git;
use super::submit::submit;
use super::transform::TransformedPipeline;
use super::Pipeline;

pub async fn watch(repository: &str, interval_mins: u64) -> Result<()> {
    let secs = interval_mins * 60;
    let mut interval = time::interval(time::Duration::from_secs(secs));
    loop {
        interval.tick().await;
        clone_transform_submit(repository).await?;
    }
}

async fn clone_transform_submit(repository: &str) -> Result<()> {
    let dir = tempdir()?;
    let dirpath = dir.path();

    git::clone(repository, dirpath, Path::new(""))?;
    log::debug!("Cloned repository to {dirpath:?}");

    let buf = fs::read_to_string(dirpath.join("recesser.yaml")).await?;
    let pipeline: Pipeline = serde_yaml::from_str(&buf)?;
    log::debug!("Original pipeline: {pipeline:#?}");

    let transformed_pipeline = TransformedPipeline::try_from(pipeline)?;
    log::debug!("Transformed pipeline: {transformed_pipeline:#?}");

    submit(&transformed_pipeline, "").await?;
    Ok(())
}
