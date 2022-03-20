#![forbid(unsafe_code)]

mod argo_workflow;
mod repository;
mod workflow;

use std::path::Path;

use anyhow::Result;
use recesser_core::repository::CommitID;

use repository::LocalRepository;
use workflow::Workflow;

#[tokio::main]
async fn main() -> Result<()> {
    let url = "git@github.com:recesser/recesser.git";
    let private_key_path = Path::new("recesser.key");
    let local_repository = LocalRepository::from_remote(url, private_key_path)?;
    println!("{local_repository:#?}");
    if &CommitID::new(None) != local_repository.last_commit() {
        println!("Last commit of repository has changed.");
    }
    let workflow = Workflow::from_repo(&local_repository).await?;
    Ok(())
}
