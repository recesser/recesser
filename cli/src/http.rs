use std::fs;
use std::path::Path;

use anyhow::Result;
use recesser_core::{metadata::Metadata, repository::NewRepository, repository::Repository};
use reqwest::blocking::{self, multipart, Response};
use reqwest::StatusCode;

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

    fn download_and_save_file(&self, url: &str, filepath: &Path) -> Result<()> {
        let mut file_resp = self.client.get(url).send()?;
        let mut file = fs::File::create(filepath)?;
        file_resp.copy_to(&mut file)?;
        Ok(())
    }

    fn url(&self, path: &str) -> String {
        format!("{addr}{path}", addr = self.addr)
    }
}

pub trait ArtifactEndpoints {
    fn upload_file(&self, handle: &str, metadata: Metadata, filepath: &Path) -> Result<()>;
    fn list(&self) -> Result<Vec<String>>;
    fn download_file(&self, handle: &str, filepath: &Path) -> Result<()>;
    fn download_metadata(&self, handle: &str, filepath: &Path) -> Result<()>;
    fn delete(&self, handle: &str) -> Result<()>;
}

impl ArtifactEndpoints for Client {
    fn upload_file(&self, handle: &str, metadata: Metadata, filepath: &Path) -> Result<()> {
        let file = fs::File::open(filepath)?;

        let form = multipart::Form::new()
            .text("handle", String::from(handle))
            .text("metadata", serde_json::to_string(&metadata)?)
            .part("file", multipart::Part::reader(file));

        let resp = self.client.put(self.url(A)).multipart(form).send()?;
        check_body(resp)?;
        Ok(())
    }

    fn list(&self) -> Result<Vec<String>> {
        let resp = self.client.get(self.url(A)).send()?;
        let body = check_body(resp)?;
        let list: Vec<String> = serde_json::from_slice(&body)?;
        Ok(list)
    }

    fn download_file(&self, handle: &str, filepath: &Path) -> Result<()> {
        self.download_and_save_file(&self.url(&format!("{A}/{handle}/file")), filepath)?;
        Ok(())
    }

    fn download_metadata(&self, handle: &str, filepath: &Path) -> Result<()> {
        self.download_and_save_file(&self.url(&format!("{A}/{handle}/metadata")), filepath)?;
        Ok(())
    }

    fn delete(&self, handle: &str) -> Result<()> {
        let resp = self
            .client
            .delete(self.url(&format!("{A}/{handle}")))
            .send()?;
        match resp.status() {
            StatusCode::ACCEPTED => Ok(()),
            StatusCode::NOT_FOUND => anyhow::bail!("Artifact {handle} doesn't exist."),
            _ => anyhow::bail!("Internal error: {}", resp.text()?),
        }
    }
}

pub trait RepositoryEndpoints {
    fn add(&self, new_repository: &NewRepository) -> Result<()>;
    fn list(&self) -> Result<Vec<Repository>>;
    fn show(&self, name: &str) -> Result<Repository>;
    fn credentials(&self, name: &str) -> Result<()>;
    fn delete(&self, name: &str) -> Result<()>;
}

impl RepositoryEndpoints for Client {
    fn add(&self, new_repository: &NewRepository) -> Result<()> {
        let resp = self
            .client
            .put(self.url(R))
            .body(serde_json::to_vec(new_repository)?)
            .send()?;
        check_body(resp)?;
        Ok(())
    }

    fn list(&self) -> Result<Vec<Repository>> {
        let resp = self.client.get(self.url(R)).send()?;
        let body = check_body(resp)?;
        let repos: Vec<Repository> = serde_json::from_slice(&body)?;
        Ok(repos)
    }

    fn show(&self, name: &str) -> Result<Repository> {
        let resp = self.client.get(self.url(&format!("{R}/{name}"))).send()?;
        let body = check_body(resp)?;
        let repo: Repository = serde_json::from_slice(&body)?;
        Ok(repo)
    }

    fn credentials(&self, name: &str) -> Result<()> {
        let _resp = self
            .client
            .get(self.url(&format!("{R}/{name}/credentials")))
            .send()?;
        Ok(())
    }

    fn delete(&self, name: &str) -> Result<()> {
        let resp = self
            .client
            .delete(self.url(&format!("{R}/{name}")))
            .send()?;

        check_body(resp)?;
        Ok(())
    }
}

pub trait UserEndpoints {
    fn create(&self) -> Result<String>;
    fn list(&self) -> Result<Vec<String>>;
    fn revoke(&self, id: &str) -> Result<()>;
}

impl UserEndpoints for Client {
    fn create(&self) -> Result<String> {
        let resp = self.client.post(self.url(U)).send()?;
        let body = check_body(resp)?;
        Ok(String::from_utf8(body)?)
    }

    fn list(&self) -> Result<Vec<String>> {
        let resp = self.client.get(self.url(U)).send()?;
        let body = check_body(resp)?;
        let users: Vec<String> = serde_json::from_slice(&body)?;
        Ok(users)
    }

    fn revoke(&self, name: &str) -> Result<()> {
        let resp = self
            .client
            .delete(self.url(&format!("{U}/{name}")))
            .send()?;
        check_body(resp)?;
        Ok(())
    }
}

fn check_body(resp: Response) -> Result<Vec<u8>> {
    if !resp.status().is_success() {
        anyhow::bail!(resp.text()?)
    }
    Ok(resp.bytes()?.to_vec())
}
