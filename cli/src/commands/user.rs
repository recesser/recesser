use anyhow::Result;

use crate::commands::Global;
use crate::http::UserEndpoints;
use crate::parser::UserCommands;

impl UserCommands {
    pub fn call(self, global: Global) -> Result<()> {
        match self {
            UserCommands::Create => create(global)?,
            UserCommands::List => list(global)?,
            UserCommands::Revoke { id } => revoke(global, &id)?,
        }
        Ok(())
    }
}

fn create(g: Global) -> Result<()> {
    let token = g.http.create()?;
    println!("{token}");
    Ok(())
}

fn list(g: Global) -> Result<()> {
    let users = g.http.list()?;
    for user in users {
        println!("{user}");
    }
    Ok(())
}

fn revoke(g: Global, id: &str) -> Result<()> {
    g.http.revoke(id)?;
    println!("Successfully revoked acces for user: {id}");
    Ok(())
}
