use anyhow::Result;
use redis::AsyncCommands;

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

    pub async fn set(&mut self) -> Result<()> {
        self.connection.set("my_key", "hellow").await?;
        Ok(())
    }
}
