use anyhow::Result;

use crate::commands::Global;
use crate::parser::RepositoryCommands;
use crate::ssh_keys;

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
    let keypair = ssh_keys::KeyPair::generate(name)?;
    println!("{keypair:#?}");
    print!("{}", String::from_utf8(keypair.public_key)?);
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
