use std::fs;
use std::path::Path;

use anyhow::Result;
use recesser_core::metadata::Metadata;
use reqwest::blocking::{self, multipart, Response};
pub use reqwest::StatusCode;

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

    pub fn upload(&self, handle: &str, metadata: Metadata, filepath: &Path) -> Result<Response> {
        let file = fs::File::open(filepath)?;

        let form = multipart::Form::new()
            .text("handle", String::from(handle))
            .text("metadata", serde_json::to_string(&metadata)?)
            .part("file", multipart::Part::reader(file));

        let resp = self
            .client
            .put(self.url("/artifacts"))
            .multipart(form)
            .send()?;

        log::debug!("Received response: {resp:#?}");
        Ok(resp)
    }

    pub fn list(&self) -> Result<Response> {
        let resp = self.client.get(self.url("/artifacts")).send()?;
        log::debug!("Received response: {resp:#?}");
        Ok(resp)
    }

    pub fn download_file(&self, handle: &str) -> Result<Response> {
        let resp = self
            .client
            .get(self.url(&format!("/artifacts/{handle}/file")))
            .send()?;
        log::debug!("Received response: {resp:#?}");
        Ok(resp)
    }

    pub fn download_metadata(&self, handle: &str) -> Result<Response> {
        let resp = self
            .client
            .get(self.url(&format!("/artifacts/{handle}/metadata")))
            .send()?;
        log::debug!("Received response: {resp:#?}");
        Ok(resp)
    }

    pub fn delete(&self, handle: &str) -> Result<Response> {
        let resp = self
            .client
            .delete(self.url(&format!("/artifacts/{handle}")))
            .send()?;
        log::debug!("Received response: {resp:#?}");
        Ok(resp)
    }

    fn url(&self, path: &str) -> String {
        format!("{addr}{path}", addr = self.addr)
    }
}
