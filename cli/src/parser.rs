use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "rcssr", version)]
pub struct Cli {
    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Get and set configuration options
    Config { key: String, value: Option<String> },
    /// Upload file
    Upload {
        file: PathBuf,

        #[clap(short, long)]
        metadata: Option<PathBuf>,
    },
    /// Download file
    Download { handle: String },
    /// Compute file handle
    Hash { file: PathBuf },
}
