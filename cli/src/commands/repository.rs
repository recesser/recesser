use std::io::{self, BufWriter, Write};

use anyhow::Result;
use recesser_core::repository::NewRepository;

use crate::commands::Global;
use crate::http::RepositoryEndpoints;
use crate::parser::{self, RepositoryCommands};
use crate::ssh::{self, KeyGen};

impl RepositoryCommands {
    pub fn call(self, global: Global) -> Result<()> {
        match self {
            RepositoryCommands::Add { name } => add(global, &name)?,
            RepositoryCommands::List => list(global)?,
            RepositoryCommands::Show { name } => show(global, &name)?,
            RepositoryCommands::Remove { names } => remove(global, names)?,
        }
        Ok(())
    }
}

fn add(g: Global, name: &str) -> Result<()> {
    let keypair = ssh::KeyPair::generate()?;
    let pub_key = keypair.public_key.public_key.clone();
    let new_repository = NewRepository {
        name: String::from(name),
        keypair,
    };

    g.http.add(&new_repository)?;
    print!("{pub_key}");
    Ok(())
}

fn list(g: Global) -> Result<()> {
    let mut writer = BufWriter::new(io::stdout());

    let repos = g.http.list()?;
    for r in repos {
        writeln!(writer, "{}", r.name)?
    }

    writer.flush()?;
    Ok(())
}

fn show(g: Global, name: &str) -> Result<()> {
    let repo = g.http.show(name)?;
    println!("{:#?}", repo);
    Ok(())
}

fn remove(g: Global, names: Vec<String>) -> Result<()> {
    let names = parser::read_lines_from_stdin_if_emtpy(names);
    let mut writer = BufWriter::new(io::stdout());

    for name in names {
        match g.http.delete(&name) {
            Ok(_) => writeln!(writer, "Removed {name}")?,
            Err(_) => writeln!(writer, "Failed to remove {name}")?,
        }
    }

    writer.flush()?;
    Ok(())
}
