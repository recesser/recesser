use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "rcssr", version)]
pub struct Cli {
    /// Path to config file
    #[clap(long)]
    pub config: Option<PathBuf>,
    /// Print verbose output
    #[clap(short, long)]
    pub verbose: bool,
    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Get and set configuration options
    Config { key: String, value: Option<String> },
    /// Manage artifacts
    #[clap(subcommand)]
    Artifact(ArtifactCommands),
    /// Manage repositories
    #[clap(subcommand)]
    Repository(RepositoryCommands),
}

#[derive(Subcommand, Debug)]
pub enum ArtifactCommands {
    /// Compute handle
    Hash { file: PathBuf },
    /// Upload artifact
    Upload {
        file: PathBuf,

        #[clap(short, long)]
        metadata: Option<PathBuf>,
    },
    /// List all artifacts
    List,
    /// Download artifact
    Download { handle: String },
    /// Delete artifact
    Delete { handle: String },
}

#[derive(Subcommand, Debug)]
pub enum RepositoryCommands {
    /// Add repository
    Add { name: String },
    /// List all repositories
    List,
    /// Display information about repository
    Show { name: String },
    /// Remove repository
    Remove { name: String },
}
