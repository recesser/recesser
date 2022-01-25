use anyhow::Result;
use recesser_core::metadata::Metadata;
use serde::Serialize;
use thiserror::Error;

const INDEX_NAME: &str = "fileCAIdx";
const ATTRIBUTE_NAME: &str = "fileContentAddress";

#[derive(Debug, Error)]
#[error("Key {key} doesn't exist.")]
pub struct KeyNotFoundError {
    pub key: String,
}

impl KeyNotFoundError {
    pub fn new(key: &str) -> Self {
        Self {
            key: String::from(key),
        }
    }
}

#[derive(Clone)]
pub struct Database {
    connection: redis::aio::MultiplexedConnection,
}

impl Database {
    pub async fn new(addr: &str) -> Result<Self> {
        let client = redis::Client::open(addr)?;
        let mut database = Self {
            connection: client.get_multiplexed_tokio_connection().await?,
        };
        log::info!("Connected to database at: {}.", addr);
        match database.create_index().await {
            Ok(_) => log::info!("Created index."),
            Err(e) => log::info!("Failed to create index: {e}."),
        }
        Ok(database)
    }

    pub async fn set(&mut self, key: &str, value: &impl Serialize) -> Result<()> {
        redis::cmd("JSON.SET")
            .arg(key)
            .arg(".")
            .arg(serde_json::to_string(value)?)
            .query_async(&mut self.connection)
            .await?;
        Ok(())
    }

    pub async fn get(&mut self, key: &str) -> Result<Metadata> {
        let result: Option<String> = redis::cmd("JSON.GET")
            .arg(key)
            .query_async(&mut self.connection)
            .await?;
        let result = result.ok_or_else(|| KeyNotFoundError::new(key))?;
        deserialize(&result)
    }

    pub async fn keys(&mut self) -> Result<Vec<String>> {
        let result: Vec<String> = redis::cmd("KEYS")
            .arg("*")
            .query_async(&mut self.connection)
            .await?;
        log::debug!("Redis result: {result:#?}");
        Ok(result)
    }

    pub async fn delete(&mut self, key: &str) -> Result<i32> {
        let result: i32 = redis::cmd("JSON.DEL")
            .arg(key)
            .query_async(&mut self.connection)
            .await?;
        if result <= 0 {
            return Err(KeyNotFoundError::new(key).into());
        }
        Ok(result)
    }

    async fn create_index(&mut self) -> Result<()> {
        redis::cmd("FT.CREATE")
            .arg(INDEX_NAME)
            .arg(&["ON", "JSON", "SCHEMA", "$.file_content_address"])
            .arg(&["AS", ATTRIBUTE_NAME])
            .arg("TEXT")
            .query_async(&mut self.connection)
            .await?;
        Ok(())
    }

    pub async fn search(&mut self, key: &str) -> Result<Vec<String>> {
        let result: Vec<String> = redis::cmd("FT.SEARCH")
            .arg(INDEX_NAME)
            .arg(format!("'@{ATTRIBUTE_NAME}:({key})'"))
            .query_async(&mut self.connection)
            .await?;
        Ok(result)
    }
}

fn deserialize(buf: &str) -> Result<Metadata> {
    Ok(serde_json::from_str(buf)?)
}
