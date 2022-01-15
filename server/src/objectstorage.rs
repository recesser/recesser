use std::convert::AsRef;
use std::path::Path;

use anyhow::Result;
use s3::creds::Credentials;
use s3::region::Region;
use s3::bucket::Bucket;
use tokio::fs;

pub struct ObjectStorage {
    client: Bucket
}

impl ObjectStorage {
    pub fn new() -> Result<Self> {
        let region = Region::Custom {
            region: String::new(),
            endpoint: String::from("http://127.0.0.1:9000"),
        };
        let credentials = Credentials::from_env_specific(
            Some("AWS_ACCESS_KEY_ID"),
            Some("AWS_SECRET_ACCESS_KEY"),
            None,
            None,
        )?;
        let client = ObjectStorage {
            client: Bucket::new_with_path_style("artifacts", region, credentials)?
        };
        Ok(client)
    }

    pub async fn upload(&self, artifact_id: &str, data: &[u8]) -> Result<()> {
        let (_, code) = self.client.put_object(artifact_id, data).await?;
        println!("{}", code);
        Ok(())
    }

    pub async fn upload_file(&self, artifact_id: impl AsRef<str>, path: impl AsRef<Path>) -> Result<()> {
        let mut file = fs::File::open(path).await?;
        let code = self.client.put_object_stream(&mut file, artifact_id).await?;
        println!("{}", code);
        Ok(())
    }

}
