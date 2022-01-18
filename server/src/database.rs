use anyhow::Result;
use serde::Serialize;

#[derive(Clone)]
pub struct Database {
    connection: redis::aio::MultiplexedConnection,
}

impl Database {
    pub async fn new() -> Result<Self> {
        let client = redis::Client::open("redis://127.0.0.1/")?;
        let database = Database {
            connection: client.get_multiplexed_tokio_connection().await?,
        };
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

    pub async fn get(&mut self, key: &str) -> Result<()> {
        let result: String = redis::cmd("JSON.GET")
            .arg(key)
            .query_async(&mut self.connection)
            .await?;
        Ok(serde_json::from_str(&result)?)
    }
}
