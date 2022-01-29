mod compress;
mod http;
mod parser;
mod settings;

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process;

use anyhow::Result;
use chrono::{Local, NaiveDateTime};
use clap::Parser;
use recesser_core::hash::{hash, hash_from_disk};
use recesser_core::metadata::Metadata;

use http::{Client, StatusCode};
use parser::{Cli, Commands};
use settings::Settings;

fn main() -> Result<()> {
    let cli = Cli::parse();
    if let Err(e) = cli.call() {
        eprintln!("{}", e);
        process::exit(1)
    }
    Ok(())
}

struct Global {
    http: Client,
}

impl Cli {
    fn call(self) -> Result<()> {
        let s = Settings::new(&self.config)?;

        env_logger::Builder::new()
            .filter(
                None,
                match self.verbose {
                    true => log::LevelFilter::Debug,
                    false => log::LevelFilter::Info,
                },
            )
            .format(|buf, record| writeln!(buf, "{}", record.args()))
            .init();

        let global = Global {
            http: Client::new(&s.addr),
        };

        match self.commands {
            Commands::Hash { file } => hash_command(file)?,
            Commands::Upload { file, metadata } => upload_command(global, &file, metadata)?,
            Commands::List {} => list_command(global)?,
            Commands::Download { handle } => download_command(global, &handle)?,
            Commands::Delete { handle } => delete_command(global, &handle)?,
            Commands::Compress { file } => compress_command(&file)?,
            _ => println!("Not implemented"),
        };
        Ok(())
    }
}

fn hash_command(filepath: PathBuf) -> Result<()> {
    let hash = hash_from_disk(&filepath)?;
    println!("{}", hash);
    Ok(())
}

fn upload_command(g: Global, filepath: &Path, metadata_path: Option<PathBuf>) -> Result<()> {
    let file_content_address = hash_from_disk(filepath)?;
    log::debug!("File content address: {file_content_address}");

    let custom_metadata = metadata_path.map(read_custom_metadata).transpose()?;
    let metadata = Metadata {
        file_content_address,
        created: Some(Local::now().naive_utc()),
        file_created: Some(file_modified(filepath)?),
        custom: custom_metadata,
    };
    log::debug!("Metadata: {metadata:#?}");

    let handle = hash(&serde_json::to_vec(&metadata)?);
    g.http.upload(&handle, metadata, filepath)?;
    println!("{handle}");

    Ok(())
}

fn file_modified(filepath: &Path) -> Result<NaiveDateTime> {
    let metadata = fs::metadata(filepath)?;
    let created = chrono::DateTime::<chrono::Utc>::from(metadata.modified()?);
    Ok(created.naive_utc())
}

fn read_custom_metadata(filepath: PathBuf) -> Result<serde_json::Value> {
    let file = fs::File::open(filepath)?;
    Ok(serde_json::from_reader(file)?)
}

fn list_command(g: Global) -> Result<()> {
    let resp = g.http.list()?;
    let list: Vec<String> = serde_json::from_slice(&resp.bytes()?)?;
    for handle in list {
        println!("{handle}");
    }
    Ok(())
}

fn download_command(g: Global, handle: &str) -> Result<()> {
    let mut file_resp = g.http.download_file(handle)?;
    let mut file = fs::File::create(handle)?;
    file_resp.copy_to(&mut file)?;

    let mut metadata_resp = g.http.download_metadata(handle)?;
    let mut file = fs::File::create(format!("{handle}.meta.json"))?;
    metadata_resp.copy_to(&mut file)?;

    println!("Downloaded artifact: {handle}");
    Ok(())
}

fn delete_command(g: Global, handle: &str) -> Result<()> {
    let resp = g.http.delete(handle)?;
    match resp.status() {
        StatusCode::ACCEPTED => println!("Successfully deleted artifact {handle}"),
        StatusCode::NOT_FOUND => println!("Artifact {handle} doesn't exist."),
        _ => println!("Internal error: {resp:?}"),
    }
    Ok(())
}

fn compress_command(filepath: &Path) -> Result<()> {
    compress::compress_on_disk(filepath)
}
