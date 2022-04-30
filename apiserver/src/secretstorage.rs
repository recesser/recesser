use std::convert::TryInto;

use anyhow::Result;
use recesser_core::encoding::base64;
use recesser_core::repository::KeyPair;
use reqwest::Client;
use reqwest::{header, Response};
use ring::digest::SHA256_OUTPUT_LEN;
use serde::{Deserialize, Serialize};

use crate::encryption::KEY_LEN;

#[derive(Clone)]
pub struct SecretStorage {
    addr: String,
    client: Client,
}

#[derive(Deserialize)]
struct SecretResponse {
    data: Secret,
}

#[derive(Deserialize, Serialize)]
struct Secret {
    data: Data,
}

#[derive(Deserialize, Serialize)]
struct Data {
    value: String,
}

impl Secret {
    pub fn from_slice(value: &[u8]) -> Self {
        Self {
            data: Data {
                value: base64::encode(value),
            },
        }
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        base64::decode(&self.data.value)
    }
}

impl SecretStorage {
    pub fn new(addr: &str, token: String) -> Result<Self> {
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
        format!("{addr}/v1{path}", addr = self.addr)
    }

    pub async fn setup(&self) -> Result<()> {
        let resp = self.client.post(self.url("/secret/config")).send().await?;
        check_body(resp).await?;
        tracing::info!(addr = %self.addr, "Connected to secret storage");
        Ok(())
    }

    pub async fn get_ssh_key(&self, fingerprint: &str) -> Result<String> {
        let base64_fingerprint = base64::encode(fingerprint.as_bytes());
        let key = self.get(&format!("ssh_keys/{base64_fingerprint}")).await?;
        Ok(String::from_utf8(key)?)
    }

    pub async fn store_ssh_key(&self, key_pair: &KeyPair) -> Result<()> {
        let base64_fingerprint =
            base64::encode(key_pair.public_key.fingerprint.as_str().as_bytes());
        self.set(
            &format!("ssh_keys/{base64_fingerprint}"),
            key_pair.private_key.as_str().as_bytes(),
        )
        .await?;
        tracing::info!(%key_pair.public_key.fingerprint, "Stored new SSH key in secret storage");
        Ok(())
    }

    pub async fn get_hmac_key(&self) -> Result<[u8; SHA256_OUTPUT_LEN]> {
        let key = self.get("hmac_key").await?;
        Ok(key[..SHA256_OUTPUT_LEN].try_into()?)
    }

    pub async fn store_hmac_key(&self, hmac_key: &[u8; SHA256_OUTPUT_LEN]) -> Result<()> {
        self.set("hmac_key", hmac_key).await
    }

    pub async fn get_encryption_key(&self, handle: &str) -> Result<[u8; KEY_LEN]> {
        let key = self.get(&format!("encryption_key/{handle}")).await?;
        Ok(key[..KEY_LEN].try_into()?)
    }

    pub async fn store_encryption_key(&self, handle: &str, key: &[u8; KEY_LEN]) -> Result<()> {
        self.set(&format!("encryption_key/{handle}"), key).await?;
        Ok(())
    }

    async fn get(&self, key: &str) -> Result<Vec<u8>> {
        let resp = self
            .client
            .get(self.url(&format!("/secret/data/{key}")))
            .send()
            .await?;
        let body = check_body(resp).await?;
        let secret_response: SecretResponse = serde_json::from_slice(&body)?;
        secret_response.data.to_vec()
    }

    async fn set(&self, key: &str, value: &[u8]) -> Result<()> {
        let resp = self
            .client
            .post(self.url(&format!("/secret/data/{key}")))
            .json(&Secret::from_slice(value))
            .send()
            .await?;
        check_body(resp).await?;
        Ok(())
    }
}

async fn check_body(resp: Response) -> Result<Vec<u8>> {
    if !resp.status().is_success() {
        anyhow::bail!(resp.text().await?)
    }
    Ok(resp.bytes().await?.to_vec())
}
