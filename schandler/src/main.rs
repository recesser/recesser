#![forbid(unsafe_code)]

mod apiserver;
mod argo_workflows;
mod pipeline;
mod repository;
mod settings;

use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, Result};
use recesser_core::repository::Repository;
use tracing::Level;
use tracing_subscriber::filter::LevelFilter;

use apiserver::Apiserver;
use argo_workflows::{ArgoWorkflowsServer, SSHSecret, Workflow};
use pipeline::Pipeline;
use repository::LocalRepository;
use settings::Settings;

struct Global {
    apiserver: Apiserver,
    argo_workflows: ArgoWorkflowsServer,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize settings
    let s = Settings::new()?;

    // Setup logging
    let log_level = LevelFilter::from_str(&s.log_level)?;
    tracing_subscriber::fmt().with_max_level(log_level).init();

    // Initialize global state
    let apiserver_token = std::env::var("RECESSER_APISERVER_TOKEN")
        .map_err(|_| anyhow!("Apiserver token needs to be specified via environment"))?;
    let global = Arc::new(Global {
        apiserver: Apiserver::new(&s.apiserver_addr, &apiserver_token)?,
        argo_workflows: ArgoWorkflowsServer::new(&s.argo_workflows_addr),
    });

    // Poll all repositories on an interval
    let mut interval = tokio::time::interval(Duration::from_secs(s.polling_interval * 60));
    loop {
        interval.tick().await;
        poll_all_repositories(global.clone()).await?;
    }
}

#[tracing::instrument(skip_all, err(Display))]
async fn poll_all_repositories(g: Arc<Global>) -> Result<()> {
    let repositories = g.apiserver.list_repositories().await?;
    tracing::event!(Level::INFO, "Retrieved list of all repositories");

    // Start a task for each repository
    let mut handles = Vec::new();
    for repo in repositories {
        let g = g.clone();
        handles.push(tokio::spawn(async move { poll_repository(g, repo).await }));
    }

    // Wait for all handles to join
    for handle in handles {
        let _ = handle.await?;
    }

    Ok(())
}

#[tracing::instrument(skip_all, err(Display), fields(name = %repository.name))]
async fn poll_repository(g: Arc<Global>, repository: Repository) -> Result<()> {
    let private_key = g.apiserver.get_ssh_key(&repository.name).await?;
    tracing::event!(
        Level::INFO,
        message = "Retrieved private key from secret storage",
        fingerprint = %repository.public_key.fingerprint
    );

    let local_repository = LocalRepository::from_remote(&repository.url, &private_key)?;
    tracing::event!(Level::INFO, message = "Cloned repository from remote");

    if repository.last_commit == local_repository.last_commit {
        tracing::event!(
            Level::INFO,
            message = "Last commit of repository has not changed",
            commit_id = %local_repository.last_commit
        );
        return Ok(());
    }
    tracing::event!(
        Level::INFO,
        message = "Last commit of repository has changed",
        old_commit_id = %repository.last_commit,
        new_commit_id = %local_repository.last_commit
    );

    let ssh_secret_name = repository.public_key.fingerprint.to_string();
    let _ssh_secret = SSHSecret::new(ssh_secret_name.clone(), private_key);

    let pipeline = Pipeline::from_repo(&local_repository).await?;
    let workflow = Workflow::from_pipeline(pipeline, repository, ssh_secret_name)?;
    g.argo_workflows.submit(workflow).await?;

    tracing::event!(Level::INFO, message = "Successfully polled repository");
    Ok(())
}
