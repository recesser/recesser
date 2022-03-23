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
            UserCommands::RotateKey => rotate_key(global)?,
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

fn rotate_key(g: Global) -> Result<()> {
    let new_root_token = g.http.rotate_key()?;
    println!("Successfully rotated signing key and revoked access for all users");
    println!("{new_root_token}");
    Ok(())
}
