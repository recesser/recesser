use anyhow::Result;
use futures_util::TryStreamExt;
use mongodb::bson;

use recesser_core::user::User;

#[derive(Clone)]
pub struct UserStore {
    collection: mongodb::Collection<User>,
}

impl UserStore {
    pub fn new(collection: mongodb::Collection<User>) -> Self {
        Self { collection }
    }

    pub async fn create(&self, user: &User) -> Result<()> {
        self.collection.insert_one(user, None).await?;
        Ok(())
    }

    pub async fn list(&self) -> Result<Vec<User>> {
        let cursor = self.collection.find(None, None).await?;
        let users: Vec<User> = cursor.try_collect().await?;
        Ok(users)
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        self.collection
            .find_one_and_delete(bson::doc! {"id": id}, None)
            .await?;
        Ok(())
    }
}
