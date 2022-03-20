mod metadata;
mod repository;
mod user;

use anyhow::Result;
use thiserror::Error;

use metadata::MetadataStore;
use repository::RepositoryStore;
use user::UserStore;

#[derive(Clone)]
pub struct Database {
    pub repositories: RepositoryStore,
    pub metadata: MetadataStore,
    pub user: UserStore,
}

impl Database {
    pub async fn new(addr: &str) -> Result<Self> {
        let client = mongodb::Client::with_uri_str(addr).await?;
        log::info!("Connected to database at: {}.", addr);
        let db = client.database("recesser");
        Ok(Self {
            repositories: RepositoryStore::new(db.collection("repositories")),
            metadata: MetadataStore::new(db.collection("metadata")),
            user: UserStore::new(db.collection("user")),
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
