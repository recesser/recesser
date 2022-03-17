mod metadata;
mod repository;

use anyhow::Result;
use thiserror::Error;

use metadata::MetadataStore;
use repository::RepositoryStore;

#[derive(Clone)]
pub struct Database {
    pub repositories: RepositoryStore,
    pub metadata: MetadataStore,
}

impl Database {
    pub async fn new(addr: &str) -> Result<Self> {
        let client = mongodb::Client::with_uri_str(addr).await?;
        log::info!("Connected to database at: {}.", addr);
        let db = client.database("recesser");
        Ok(Self {
            repositories: RepositoryStore::new(db.collection("repositories")),
            metadata: MetadataStore::new(db.collection("metadata")),
        })
    }
}

#[derive(Debug, Error)]
#[error("Handle {handle} doesn't exist.")]
pub struct HandleNotFoundError {
    pub handle: String,
}

impl HandleNotFoundError {
    pub fn new(handle: &str) -> Self {
        Self {
            handle: String::from(handle),
        }
    }
}
