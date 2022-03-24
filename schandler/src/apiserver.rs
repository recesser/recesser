use anyhow::Result;
use recesser_core::repository::Repository;
use reqwest::{header, Client, Response};

pub struct Apiserver {
    addr: String,
    client: Client,
}

impl Apiserver {
    pub fn new(addr: &str, token: &str) -> Result<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            format!("Bearer {}", token).try_into()?,
        );
        let cb = Client::builder().default_headers(headers);
        Ok(Self {
            addr: String::from(addr),
            client: cb.build()?,
        })
    }

    fn url(&self, path: &str) -> String {
        format!("{addr}{path}", addr = self.addr)
    }

    pub async fn list_repositories(&self) -> Result<Vec<Repository>> {
        let resp = self.client.get(self.url("/repositories")).send().await?;
        let body = check_body(resp).await?;
        let repos: Vec<Repository> = serde_json::from_slice(&body)?;
        Ok(repos)
    }

    pub async fn get_ssh_key(&self, name: &str) -> Result<String> {
        let resp = self
            .client
            .get(self.url(&format!("/repositories/{name}/credentials")))
            .send()
            .await?;
        let body = check_body(resp).await?;
        Ok(String::from_utf8(body)?)
    }
}

async fn check_body(resp: Response) -> Result<Vec<u8>> {
    if !resp.status().is_success() {
        anyhow::bail!(resp.text().await?)
    }
    Ok(resp.bytes().await?.to_vec())
}
