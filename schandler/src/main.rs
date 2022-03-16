mod argo_workflow;
mod repository;
mod workflow;

use std::path::Path;

use anyhow::Result;
use recesser_core::repository::Repository;

use repository::LocalRepository;
use workflow::Workflow;

#[tokio::main]
async fn main() -> Result<()> {
    let repository = Repository::new("recesser/recesser");
    println!("{repository:#?}");
    let private_key_path = Path::new("recesser.key");
    let local_repository = LocalRepository::from_remote(&repository, private_key_path)?;
    println!("{local_repository:#?}");
    if repository.last_commit() != local_repository.last_commit() {
        println!("Last commit of repository has changed.");
    }
    let workflow = Workflow::from_repo(&local_repository).await?;
    Ok(())
}
