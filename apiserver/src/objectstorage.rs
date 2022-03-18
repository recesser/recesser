use std::convert::AsRef;
use std::path::Path;

use anyhow::Result;
use s3::creds::Credentials;
use s3::region::Region;
use s3::{Bucket, BucketConfiguration};
use tokio::fs;

const BUCKET_NAME: &str = "artifacts";

#[derive(Clone)]
pub struct ObjectStorage {
    bucket: Bucket,
}

impl ObjectStorage {
    pub async fn new(addr: &str) -> Result<Self> {
        let region = Region::Custom {
            region: String::new(),
            endpoint: String::from(addr),
        };

        let credentials = Credentials::from_env_specific(
            Some("RECESSER_OBJECTSTORAGE_USER"),
            Some("RECESSER_OBJECTSTORAGE_PASSWORD"),
            None,
            None,
        )?;

        let create_bucket_response = Bucket::create_with_path_style(
            BUCKET_NAME,
            region.clone(),
            credentials.clone(),
            BucketConfiguration::default(),
        )
        .await?;

        let bucket = match create_bucket_response.success() {
            true => create_bucket_response.bucket,
            false => Bucket::new_with_path_style(BUCKET_NAME, region, credentials)?,
        };

        Ok(ObjectStorage { bucket })
    }

    pub async fn upload_file(
        &self,
        content_address: impl AsRef<str>,
        file_path: impl AsRef<Path>,
    ) -> Result<()> {
        let mut file = fs::File::open(file_path).await?;
        let code = self
            .bucket
            .put_object_stream(&mut file, content_address)
            .await?;
        log::debug!("Received minio code: {code}");
        Ok(())
    }

    pub async fn download_file(
        &self,
        content_address: impl AsRef<str>,
        filepath: &Path,
    ) -> Result<()> {
        let mut file = fs::File::create(&filepath).await?;
        let code = self
            .bucket
            .get_object_stream(content_address, &mut file)
            .await?;
        log::debug!("Received minio code: {code}");
        Ok(())
    }

    // pub async fn list(&self) -> Result<Vec<String>> {
    //     let results = self.bucket.list(String::from(""), None).await?;
    //     let artifacts: Vec<String> = results.iter().map(|i| String::from(&i.name)).collect();
    //     Ok(artifacts)
    // }

    pub async fn exists(&self, content_address: impl AsRef<str>) -> Result<bool> {
        let (_, code) = self.bucket.head_object(content_address).await?;
        log::debug!("Received minio code: {code}");
        Ok(!matches!(code, 404))
    }

    pub async fn delete(&self, content_address: &str) -> Result<()> {
        log::debug!("File content address for deletion {content_address}");
        let (_, code) = self.bucket.delete_object(content_address).await?;
        log::debug!("Received minio code: {code}");
        Ok(())
    }
}
