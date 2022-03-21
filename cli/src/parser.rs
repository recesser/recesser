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
    /// Revoke acccess for a user
    Revoke { id: String },
}
