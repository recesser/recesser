use std::fs;
use std::path::Path;

use anyhow::Result;
use recesser_core::{metadata::Metadata, repository::NewRepository};
use reqwest::blocking::{self, multipart, Response};
pub use reqwest::StatusCode;

const A: &str = "/artifacts";
const R: &str = "/repositories";
const U: &str = "/users";

pub struct Client {
    addr: String,
    client: blocking::Client,
}

impl Client {
    pub fn new(addr: &str) -> Self {
        Self {
            addr: String::from(addr),
            client: blocking::Client::new(),
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{addr}{path}", addr = self.addr)
    }
}

pub trait ArtifactEndpoints {
    fn upload_file(&self, handle: &str, metadata: Metadata, filepath: &Path) -> Result<Response>;
    fn list(&self) -> Result<Response>;
    fn download_file(&self, handle: &str) -> Result<Response>;
    fn download_metadata(&self, handle: &str) -> Result<Response>;
    fn delete(&self, handle: &str) -> Result<Response>;
}

impl ArtifactEndpoints for Client {
    fn upload_file(&self, handle: &str, metadata: Metadata, filepath: &Path) -> Result<Response> {
        let file = fs::File::open(filepath)?;

        let form = multipart::Form::new()
            .text("handle", String::from(handle))
            .text("metadata", serde_json::to_string(&metadata)?)
            .part("file", multipart::Part::reader(file));

        let resp = self.client.put(self.url(A)).multipart(form).send()?;
        Ok(resp)
    }

    fn list(&self) -> Result<Response> {
        let resp = self.client.get(self.url(A)).send()?;

        Ok(resp)
    }

    fn download_file(&self, handle: &str) -> Result<Response> {
        let resp = self
            .client
            .get(self.url(&format!("{A}/{handle}/file")))
            .send()?;
        Ok(resp)
    }

    fn download_metadata(&self, handle: &str) -> Result<Response> {
        let resp = self
            .client
            .get(self.url(&format!("{A}/{handle}/metadata")))
            .send()?;
        Ok(resp)
    }

    fn delete(&self, handle: &str) -> Result<Response> {
        let resp = self
            .client
            .delete(self.url(&format!("{A}/{handle}")))
            .send()?;
        Ok(resp)
    }
}

pub trait RepositoryEndpoints {
    fn register(&self, new_repository: &NewRepository) -> Result<Response>;
    fn list(&self) -> Result<Response>;
    fn show(&self, name: &str) -> Result<Response>;
    fn credentials(&self, name: &str) -> Result<Response>;
    fn delete(&self, name: &str) -> Result<Response>;
}

impl RepositoryEndpoints for Client {
    fn register(&self, new_repository: &NewRepository) -> Result<Response> {
        let resp = self
            .client
            .put(self.url(R))
            .body(serde_json::to_vec(new_repository)?)
            .send()?;
        Ok(resp)
    }

    fn list(&self) -> Result<Response> {
        let resp = self.client.get(self.url(R)).send()?;
        Ok(resp)
    }

    fn show(&self, name: &str) -> Result<Response> {
        let resp = self.client.get(self.url(&format!("{R}/{name}"))).send()?;
        Ok(resp)
    }

    fn credentials(&self, name: &str) -> Result<Response> {
        let resp = self
            .client
            .get(self.url(&format!("{R}/{name}/credentials")))
            .send()?;
        Ok(resp)
    }

    fn delete(&self, name: &str) -> Result<Response> {
        let resp = self
            .client
            .delete(self.url(&format!("{R}/{name}")))
            .send()?;
        Ok(resp)
    }
}

pub trait UserEndpoints {
    fn create(&self) -> Result<Response>;
    fn list(&self) -> Result<Response>;
    fn revoke(&self, id: &str) -> Result<Response>;
}

impl UserEndpoints for Client {
    fn create(&self) -> Result<Response> {
        let resp = self.client.post(self.url(U)).send()?;
        Ok(resp)
    }

    fn list(&self) -> Result<Response> {
        let resp = self.client.get(self.url(U)).send()?;
        Ok(resp)
    }

    fn revoke(&self, name: &str) -> Result<Response> {
        let resp = self
            .client
            .delete(self.url(&format!("{U}/{name}")))
            .send()?;
        Ok(resp)
    }
}
