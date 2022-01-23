use std::path::Path;

use anyhow::Result;
use recesser_core::metadata::Metadata;
use reqwest::blocking::{self, multipart};

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

    pub fn upload(&self, content_address: &str, metadata: Metadata, filepath: &Path) -> Result<()> {
        let form = multipart::Form::new()
            .text("content-address", String::from(content_address))
            .text("metadata", serde_json::to_string(&metadata)?)
            .file("file", filepath)?;

        let resp = self
            .client
            .post(self.url("/artifacts"))
            .multipart(form)
            .send()?;

        log::debug!("Received response: {resp:#?}");

        Ok(())
    }

    fn url(&self, path: &str) -> String {
        format!("{addr}{path}", addr = self.addr)
    }
}