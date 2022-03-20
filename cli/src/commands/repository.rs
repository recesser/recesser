use anyhow::Result;
use recesser_core::repository::NewRepository;

use crate::commands::Global;
use crate::http::RepositoryEndpoints;
use crate::parser::RepositoryCommands;
use crate::ssh::{self, KeyGen};

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
    let pub_key = keypair.public_key.public_key.clone();
    let new_repository = NewRepository {
        name: String::from(name),
        keypair,
    };

    g.http.add(&new_repository)?;
    print!("{}", pub_key);
    Ok(())
}

fn list(g: Global) -> Result<()> {
    let repos = g.http.list()?;
    for r in repos {
        println!("{}", r.name)
    }
    Ok(())
}

fn show(g: Global, name: &str) -> Result<()> {
    let repo = g.http.show(name)?;
    println!("{:#?}", repo);
    Ok(())
}

fn remove(g: Global, name: &str) -> Result<()> {
    g.http.delete(name)?;
    println!("Successfully removed repository: {name}");
    Ok(())
}
