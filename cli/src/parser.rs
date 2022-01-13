use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
/// recesser - Reproducible Computational Social Science Research
pub enum Cli {
    /// Get and set configuration options
    Config { key: String, value: Option<String> },
    /// Download file
    Download { file: PathBuf },
    /// Compute file id
    Hash { file: PathBuf },
    /// Upload file
    Upload { file: PathBuf },
}
