#![forbid(unsafe_code)]

mod apiserver;
mod argo_workflow;
mod repository;
mod workflow;

use anyhow::Result;
use recesser_core::repository::CommitID;

use apiserver::Apiserver;
use repository::LocalRepository;
use workflow::Workflow;

use crate::argo_workflow::ArgoWorkflow;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = std::env::var("RECESSER_APISERVER_ADDR")?;
    let token = std::env::var("RECESSER_APISERVER_TOKEN")?;
    let apiserver = Apiserver::new(&addr, &token)?;

    let repositories = apiserver.list_repositories().await?;
    println!("{repositories:#?}");

    let name = "recesser/tensorflow-example";
    let url = format!("git@github.com:{name}.git");
    let private_key = apiserver.get_ssh_key(name).await?;
    println!("{}", private_key);

    let local_repository = LocalRepository::from_remote(&url, &private_key)?;
    println!("{local_repository:#?}");
    if &CommitID::new(None) != local_repository.last_commit() {
        println!("Last commit of repository has changed.");
    }

    let workflow = Workflow::from_repo(&local_repository).await?;
    let _argo_workflow = ArgoWorkflow::try_from(workflow)?;

    Ok(())
}
