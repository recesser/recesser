use anyhow::Result;
use recesser_core::repository::NewRepository;

use crate::commands::Global;
use crate::http::RepositoryEndpoints;
use crate::parser::RepositoryCommands;
use crate::ssh::{self, KeyGen, ReadFingerprint};

impl RepositoryCommands {
    pub fn call(self, global: Global) -> Result<()> {
        match self {
            RepositoryCommands::Add { name } => add(global, &name)?,
            RepositoryCommands::List => list(global)?,
            RepositoryCommands::Show { name } => show(global, &name)?,
            RepositoryCommands::Remove { name } => remove(global, &name)?,
        }
        Ok(())
    }
}

fn add(g: Global, name: &str) -> Result<()> {
    let keypair = ssh::KeyPair::generate(name)?;
    let pub_key = String::from_utf8(keypair.public_key.public_key.clone())?;
    let new_repository = NewRepository {
        name: String::from(name),
        keypair,
    };
    println!("{}", pub_key);
    Ok(())
}

fn list(g: Global) -> Result<()> {
    Ok(())
}

fn show(g: Global, name: &str) -> Result<()> {
    Ok(())
}

fn remove(g: Global, name: &str) -> Result<()> {
    Ok(())
}
