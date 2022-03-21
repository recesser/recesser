mod artifact;
mod repository;
mod user;

use std::io::Write;

use anyhow::Result;

use crate::http::Client;
use crate::parser::{AdminCommands, Cli, Commands};

pub struct Global {
    http: Client,
}

impl Cli {
    pub fn call(self) -> Result<()> {
        env_logger::Builder::new()
            .filter(
                None,
                match self.verbose {
                    true => log::LevelFilter::Debug,
                    false => log::LevelFilter::Info,
                },
            )
            .format(|buf, record| writeln!(buf, "{}", record.args()))
            .init();

        let addr = match self.host {
            Some(addr) => addr,
            None => std::env::var("RECESSER_ADDR").map_err(|_| {
                anyhow::anyhow!(
                    "Host address needs to be specified via environment or as command line argument"
                )
            })?,
        };

        let token = match self.token {
            Some(token) => token,
            None => std::env::var("RECESSER_TOKEN").map_err(|_| {
                anyhow::anyhow!(
                    "Access token needs to be specified via environment or as command line argument"
                )
            })?,
        };

        let global = Global {
            http: Client::new(&addr, token),
        };

        match self.commands {
            Commands::Artifact(cmd) => cmd.call(global)?,
            Commands::Repository(cmd) => cmd.call(global)?,
            Commands::Admin(cmd) => cmd.call(global)?,
        };
        Ok(())
    }
}

impl AdminCommands {
    pub fn call(self, global: Global) -> Result<()> {
        match self {
            AdminCommands::User(cmd) => cmd.call(global)?,
        }
        Ok(())
    }
}
