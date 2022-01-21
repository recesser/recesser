use anyhow::Result;
use recesser_core::metadata::Metadata;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Key {key} doesn't exist.")]
pub struct KeyNotFoundError {
    pub key: String,
}

#[derive(Clone)]
pub struct Database {
    connection: redis::aio::MultiplexedConnection,
}

impl Database {
    pub async fn new(addr: &str) -> Result<Self> {
        let client = redis::Client::open(addr)?;
        let database = Database {
            connection: client.get_multiplexed_tokio_connection().await?,
        };
        log::info!("Connected to database at: {}", addr);
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
        let result = result.ok_or(KeyNotFoundError {
            key: String::from(key),
        })?;
        deserialize(&result)
    }

    pub async fn get_all(&mut self) -> Result<Vec<Metadata>> {
        let keys = self.keys().await?;
        let result: Vec<String> = redis::cmd("JSON.MGET")
            .arg(&keys)
            .query_async(&mut self.connection)
            .await?;
        Ok(result.iter().filter_map(|r| deserialize(r).ok()).collect())
    }

    async fn keys(&mut self) -> Result<Vec<String>> {
        let result: Vec<String> = redis::cmd("KEYS")
            .arg("*")
            .query_async(&mut self.connection)
            .await?;
        Ok(result)
    }

    pub async fn delete(&mut self, key: &str) -> Result<i32> {
        let result: i32 = redis::cmd("JSON.DEL")
            .arg(key)
            .query_async(&mut self.connection)
            .await?;
        if result <= 0 {
            return Err(KeyNotFoundError {
                key: String::from(key),
            }
            .into());
        }
        Ok(result)
    }
}

fn deserialize(buf: &str) -> Result<Metadata> {
    Ok(serde_json::from_str(buf)?)
}
