use anyhow::Result;
use futures_util::TryStreamExt;
use mongodb::bson;
use recesser_core::metadata::Metadata;
use serde::{Deserialize, Serialize};

use super::HandleNotFoundError;

#[derive(Clone)]
pub struct MetadataStore {
    collection: mongodb::Collection<MetadataDoc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetadataDoc {
    handle: String,
    metadata: Metadata,
}

impl MetadataStore {
    pub fn new(collection: mongodb::Collection<MetadataDoc>) -> Self {
        Self { collection }
    }

    pub async fn insert(&self, handle: &str, metadata: &Metadata) -> Result<()> {
        let metadata_doc = MetadataDoc {
            handle: String::from(handle),
            metadata: metadata.clone(),
        };
        self.collection.insert_one(&metadata_doc, None).await?;
        log::debug!("Inserted metadata: {:#?}", metadata_doc);
        Ok(())
    }

    pub async fn retrieve(&self, handle: &str) -> Result<Metadata> {
        let metadata_doc = self
            .collection
            .find_one(filter_handle(handle), None)
            .await?
            .ok_or_else(|| HandleNotFoundError::new(handle))?;
        log::debug!("Retrieved metadata: {:#?}", metadata_doc.metadata);
        Ok(metadata_doc.metadata)
    }

    pub async fn list_handles(&self) -> Result<Vec<String>> {
        let cursor = self.collection.find(None, None).await?;
        let metadata_docs: Vec<MetadataDoc> = cursor.try_collect().await?;
        let handles: Vec<String> = metadata_docs.into_iter().map(|x| x.handle).collect();
        log::debug!("Retrieved artifact handles: {handles:#?}");
        Ok(handles)
    }

    pub async fn delete(&self, handle: &str) -> Result<()> {
        self.collection
            .find_one_and_delete(filter_handle(handle), None)
            .await?;
        Ok(())
    }

    pub async fn search_object_handle(&self, handle: &str) -> Result<Vec<String>> {
        let cursor = self
            .collection
            .find(bson::doc! {"metadata": {"object_handle": handle} }, None)
            .await?;

        let metadata_docs: Vec<MetadataDoc> = cursor.try_collect().await?;
        let handles: Vec<String> = metadata_docs
            .into_iter()
            .map(|x| x.metadata.object_handle.to_string())
            .collect();

        log::debug!("Retrieved artifact handles with object handle: {handle}: {handles:#?}");
        Ok(handles)
    }
}

fn filter_handle(handle: &str) -> bson::Document {
    bson::doc! {"handle": handle}
}
