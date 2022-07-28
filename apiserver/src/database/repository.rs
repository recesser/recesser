use anyhow::Result;
use futures_util::TryStreamExt;
use mongodb::bson;
use recesser_core::repository::Repository;

use crate::database::DocumentNotFoundError;

#[derive(Clone)]
pub struct RepositoryStore {
    collection: mongodb::Collection<Repository>,
}

impl RepositoryStore {
    pub fn new(collection: mongodb::Collection<Repository>) -> Self {
        Self { collection }
    }

    pub async fn add(&self, repository: Repository) -> Result<()> {
        let name = repository.name.clone();
        self.collection.insert_one(repository, None).await?;
        tracing::info!(%name, "Stored new repository in database");
        Ok(())
    }

    pub async fn update_last_commit(&self, name: &str, new_commit: &str) -> Result<()> {
        tracing::debug!(
            message = "Updating last commit of repository",
            repository = name,
            new_commit = new_commit
        );
        self.collection
            .find_one_and_update(
                bson::doc! {"name": name},
                bson::doc! {"$set": {"last_commit": new_commit}},
                None,
            )
            .await?;

        Ok(())
    }

    pub async fn list(&self) -> Result<Vec<Repository>> {
        let cursor = self.collection.find(None, None).await?;
        let repositories: Vec<Repository> = cursor.try_collect().await?;
        Ok(repositories)
    }

    pub async fn show(&self, name: &str) -> Result<Repository> {
        let repository = self
            .collection
            .find_one(bson::doc! {"name": name}, None)
            .await?
            .ok_or_else(|| {
                DocumentNotFoundError::new(&format!("Repository doesn't exist: {name}"))
            })?;
        tracing::debug!(?repository);
        Ok(repository)
    }

    pub async fn remove(&self, name: &str) -> Result<()> {
        self.collection
            .find_one_and_delete(bson::doc! {"name": name}, None)
            .await?;
        Ok(())
    }
}
