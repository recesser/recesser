use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use recesser_core::handle::Handle;
use recesser_core::metadata::Metadata;

use crate::commands::Global;
use crate::http::{ArtifactEndpoints, StatusCode};
use crate::parser::ArtifactCommands;

impl ArtifactCommands {
    pub fn call(self, global: Global) -> Result<()> {
        match self {
            ArtifactCommands::Hash { file } => hash(file)?,
            ArtifactCommands::Upload { file, metadata } => upload(global, &file, metadata)?,
            ArtifactCommands::List {} => list(global)?,
            ArtifactCommands::Download { handle } => download(global, &handle)?,
            ArtifactCommands::Delete { handle } => delete(global, &handle)?,
        }
        Ok(())
    }
}

fn hash(filepath: PathBuf) -> Result<()> {
    let object_handle = Handle::compute_from_file(&filepath)?;
    println!("{object_handle}");
    Ok(())
}

fn upload(g: Global, filepath: &Path, metadata_path: Option<PathBuf>) -> Result<()> {
    let object_handle = Handle::compute_from_file(filepath)?;
    log::debug!("Object handle: {object_handle:#?}");

    let custom_metadata = metadata_path.map(read_custom_metadata).transpose()?;
    let metadata = Metadata {
        object_handle,
        custom: custom_metadata,
    };
    log::debug!("{metadata:#?}");

    let artifact_handle = Handle::compute_from_buf(&serde_json::to_vec(&metadata)?);
    g.http
        .upload_file(&artifact_handle.to_string(), metadata, filepath)?;
    println!("{artifact_handle}");

    Ok(())
}

fn read_custom_metadata(filepath: PathBuf) -> Result<serde_json::Value> {
    let file = fs::File::open(filepath)?;
    Ok(serde_json::from_reader(file)?)
}

fn list(g: Global) -> Result<()> {
    let resp = g.http.list()?;
    match resp.status().is_success() {
        true => {
            let list: Vec<String> = serde_json::from_slice(&resp.bytes()?)?;
            for handle in list {
                println!("{handle}");
            }
        }
        false => println!("{}", resp.text()?),
    }
    Ok(())
}

fn download(g: Global, handle: &str) -> Result<()> {
    let mut file_resp = g.http.download_file(handle)?;
    let mut file = fs::File::create(handle)?;
    file_resp.copy_to(&mut file)?;

    let mut metadata_resp = g.http.download_metadata(handle)?;
    let mut file = fs::File::create(format!("{handle}.meta.json"))?;
    metadata_resp.copy_to(&mut file)?;

    println!("Downloaded artifact: {handle}");
    Ok(())
}

fn delete(g: Global, handle: &str) -> Result<()> {
    let resp = g.http.delete(handle)?;
    match resp.status() {
        StatusCode::ACCEPTED => println!("Successfully deleted artifact {handle}"),
        StatusCode::NOT_FOUND => println!("Artifact {handle} doesn't exist."),
        _ => println!("Internal error: {resp:?}"),
    }
    Ok(())
}
