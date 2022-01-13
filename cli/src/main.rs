mod parser;

use std::{process, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use recesser_core::hash::hash_from_disk;

use self::parser::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    if let Err(e) = cli.call() {
        eprintln!("{}", e);
        process::exit(1)
    }
    Ok(())
}

impl Cli {
    fn call(self) -> Result<()> {
        match self {
            Cli::Hash {file} => hash(file)?,
            _ => println!("Not implemented"),
        };
        Ok(())
    }
}

fn hash(filepath: PathBuf) -> Result<()> {
    let hash = hash_from_disk(filepath)?;
    println!("{}", hash);
    Ok(())
}
