use std::fs;
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};

use anyhow::Result;
use recesser_core::handle::Handle;
use recesser_core::metadata::Metadata;

use crate::commands::Global;
use crate::http::ArtifactEndpoints;
use crate::parser::{self, ArtifactCommands};

impl ArtifactCommands {
    pub fn call(self, global: Global) -> Result<()> {
        match self {
            ArtifactCommands::Hash { file } => hash(file)?,
            ArtifactCommands::Upload { file, metadata } => upload(global, &file, metadata)?,
            ArtifactCommands::List {} => list(global)?,
            ArtifactCommands::Download { handles } => download(global, handles)?,
            ArtifactCommands::Delete { handles } => delete(global, handles)?,
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
    let mut writer = BufWriter::new(io::stdout());

    let handles = g.http.list()?;
    for handle in handles {
        writeln!(writer, "{handle}")?;
    }

    writer.flush()?;
    Ok(())
}

fn download(g: Global, handles: Vec<String>) -> Result<()> {
    let handles = parser::read_lines_from_stdin_if_emtpy(handles);
    for handle in handles {
        match download_artifact(&g, &handle) {
            // Print directly instead of writing into BufWriter to give immediate feedback once
            // a file is downloaded
            Ok(_) => println!("Downloaded {handle}"),
            Err(_) => println!("Failed to download {handle}"),
        }
    }
    Ok(())
}

fn download_artifact(g: &Global, handle: &str) -> Result<()> {
    g.http.download_file(handle, Path::new(handle))?;
    g.http
        .download_metadata(handle, Path::new(&format!("{handle}.meta.json")))?;
    Ok(())
}

fn delete(g: Global, handles: Vec<String>) -> Result<()> {
    let handles = parser::read_lines_from_stdin_if_emtpy(handles);
    let mut writer = BufWriter::new(io::stdout());

    for handle in handles {
        g.http.delete(&handle)?;
        writeln!(writer, "{handle}")?;
    }

    writer.flush()?;
    Ok(())
}
