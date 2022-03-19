#![forbid(unsafe_code)]

mod commands;
mod http;
mod parser;
mod settings;
mod ssh;

use std::process;

use anyhow::Result;
use clap::Parser;

use parser::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    if let Err(e) = cli.call() {
        eprintln!("{}", e);
        process::exit(1)
    }
    Ok(())
}
