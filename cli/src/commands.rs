mod artifact;
mod repository;
mod user;

use std::io::Write;

use anyhow::Result;

use crate::http::Client;
use crate::parser::{AdminCommands, Cli, Commands};
use crate::settings::Settings;

pub struct Global {
    http: Client,
}

impl Cli {
    pub fn call(self) -> Result<()> {
        let s = Settings::new(&self.config)?;

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

        let global = Global {
            http: Client::new(&s.addr),
        };

        match self.commands {
            Commands::Artifact(cmd) => cmd.call(global)?,
            Commands::Admin(cmd) => cmd.call(global)?,
            _ => println!("Not implemented"),
        };
        Ok(())
    }
}

impl AdminCommands {
    pub fn call(self, global: Global) -> Result<()> {
        match self {
            AdminCommands::Repository(cmd) => cmd.call(global)?,
            AdminCommands::User(cmd) => cmd.call(global)?,
        }
        Ok(())
    }
}
