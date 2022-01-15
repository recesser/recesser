use anyhow::Result;

pub struct Database {
    client: redis::Client,
}

impl Database {
    pub fn new() -> Result<Self> {
        let client = Database {
            client: redis::Client::open("redis://127.0.0.1/")?
        };
        Ok(client)
    }
}
