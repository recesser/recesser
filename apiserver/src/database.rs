mod metadata;
mod repository;
mod user;

use anyhow::{Error, Result};
use thiserror::Error;

use crate::error::UserError;
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
        tracing::info!(addr, "Connected to database");
        let db = client.database("recesser");
        Ok(Self {
            repositories: RepositoryStore::new(db.collection("repositories")),
            metadata: MetadataStore::new(db.collection("metadata")),
            user: UserStore::new(db.collection("user")),
        })
    }
}

#[derive(Debug, Error)]
#[error("{message}")]
pub struct DocumentNotFoundError {
    pub message: String,
}

impl DocumentNotFoundError {
    pub fn new(message: &str) -> Self {
        Self {
            message: String::from(message),
        }
    }

    pub fn downcast(e: Error, path: &str) -> UserError {
        match e.downcast::<Self>() {
            Ok(e) => UserError::not_found(path, e),
            Err(e) => UserError::internal(e),
        }
    }
}
