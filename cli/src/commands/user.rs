use anyhow::Result;
use recesser_core::user::Scope;

use crate::commands::Global;
use crate::http::UserEndpoints;
use crate::parser::UserCommands;

impl UserCommands {
    pub fn call(self, global: Global) -> Result<()> {
        match self {
            UserCommands::Create { scope } => create(global, scope)?,
            UserCommands::List => list(global)?,
            UserCommands::Revoke { id } => revoke(global, &id)?,
        }
        Ok(())
    }
}

fn create(g: Global, scope: Scope) -> Result<()> {
    let token = g.http.create(scope)?;
    println!("{token}");
    Ok(())
}

fn list(g: Global) -> Result<()> {
    let users = g.http.list()?;
    for user in users {
        println!("{} {:?}", user.id, user.scope);
    }
    Ok(())
}

fn revoke(g: Global, id: &str) -> Result<()> {
    g.http.revoke(id)?;
    println!("Successfully revoked access for user: {id}");
    Ok(())
}
