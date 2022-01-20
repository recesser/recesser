use std::convert::AsRef;
use std::path::{Path, PathBuf};

use anyhow::Result;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use tokio::fs;

use crate::file;

#[derive(Clone)]
pub struct ObjectStorage {
    bucket: Bucket,
}

impl ObjectStorage {
    pub fn new(addr: &str) -> Result<Self> {
        let region = Region::Custom {
            region: String::new(),
            endpoint: String::from(addr),
        };
        let credentials = Credentials::from_env_specific(
            Some("AWS_ACCESS_KEY_ID"),
            Some("AWS_SECRET_ACCESS_KEY"),
            None,
            None,
        )?;
        let objectstorage = ObjectStorage {
            bucket: Bucket::new_with_path_style("artifacts", region, credentials)?,
        };
        Ok(objectstorage)
    }

    pub async fn upload_file(
        &self,
        artifact_id: impl AsRef<str>,
        path: impl AsRef<Path>,
    ) -> Result<()> {
        let mut file = fs::File::open(path).await?;
        let code = self
            .bucket
            .put_object_stream(&mut file, artifact_id)
            .await?;
        println!("Received minio code: {}", code);
        Ok(())
    }

    pub async fn download_file(&self, artifact_id: impl AsRef<str>) -> Result<PathBuf> {
        let path = file::tempfile()?;
        let mut file = fs::File::create(&path).await?;
        let code = self
            .bucket
            .get_object_stream(&artifact_id, &mut file)
            .await?;
        println!("Received minio code: {}", code);
        Ok(path)
    }
}
