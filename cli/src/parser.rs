use std::io::{self, BufRead};
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use recesser_core::user::Scope;

#[derive(Parser, Debug)]
#[clap(name = "rcssr", version)]
pub struct Cli {
    /// Path to config file
    #[clap(long)]
    pub config: Option<PathBuf>,
    /// Print verbose output
    #[clap(short, long)]
    pub verbose: bool,
    /// URL of system
    #[clap(short, long)]
    pub host: Option<String>,
    /// Access token
    #[clap(short, long)]
    pub token: Option<String>,
    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Manage artifacts
    #[clap(subcommand)]
    Artifact(ArtifactCommands),
    /// Manage repositories
    #[clap(subcommand)]
    Repository(RepositoryCommands),
    /// Administrate system
    #[clap(subcommand)]
    Admin(AdminCommands),
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
    Download { handles: Vec<String> },
    /// Delete artifact
    Delete { handles: Vec<String> },
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
    Remove { names: Vec<String> },
}

#[derive(Subcommand, Debug)]
pub enum AdminCommands {
    /// Manage users
    #[clap(subcommand)]
    User(UserCommands),
}

#[derive(Subcommand, Debug)]
pub enum UserCommands {
    /// Create user
    Create { scope: Scope },
    /// List all users
    List,
    /// Rotate signing key and revoke acccess for all current users
    RotateKey,
}

pub fn read_lines_from_stdin_if_emtpy(vec: Vec<String>) -> Vec<String> {
    if vec.is_empty() && atty::isnt(atty::Stream::Stdin) {
        return io::stdin()
            .lock()
            .lines()
            .map(Result::unwrap_or_default)
            .collect();
    }
    vec
}
