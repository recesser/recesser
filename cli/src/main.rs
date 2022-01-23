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

use http::Client;
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
            _ => println!("Not implemented"),
        };
        Ok(())
    }
}

fn hash_command(filepath: PathBuf) -> Result<()> {
    let hash = hash_from_disk(filepath)?;
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

    let content_address = hash(&serde_json::to_vec(&metadata)?);
    println!("{content_address}");
    g.http.upload(&content_address, metadata, filepath)?;

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
