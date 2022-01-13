use anyhow::Result;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;

pub struct Backend {
    region: Region,
    credentials: Credentials,
    bucket: String,
}

impl Backend {
    pub fn bucket(name: &str) -> Result<Bucket> {
        let backend = Backend {
            region: Region::Custom {
                region: String::new(),
                endpoint: String::from("http://127.0.0.1:9000"),
            },
            credentials: Credentials::from_env_specific(
                Some("AWS_ACCESS_KEY_ID"),
                Some("AWS_SECRET_ACCESS_KEY"),
                None,
                None,
            )?,
            bucket: String::from(name),
        };
        Bucket::new_with_path_style(&backend.bucket, backend.region, backend.credentials)
    }
}
