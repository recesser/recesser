use anyhow::Result;
use ring::rand::SecureRandom;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use recesser_core::encoding::base64;

#[derive(Deserialize, Serialize, PartialEq)]
struct Token {
    id: [u8; 16],
    secret: [u8; 32],
    scope: Scope,
}

#[derive(Deserialize, Serialize, PartialEq)]
enum Scope {
    User,
    Admin,
}

impl Token {
    pub fn generate(rng: &dyn SecureRandom, scope: Scope) -> Result<Self> {
        let mut secret = [0_u8; 32];
        rng.fill(&mut secret)
            .map_err(|_| anyhow::anyhow!("Failed to generate secret."))?;
        Ok(Self {
            id: *Uuid::new_v4().as_bytes(),
            secret,
            scope,
        })
    }

    pub fn to_base64(&self) -> Result<String> {
        let b = bson::to_vec(self)?;
        let mut buf = String::new();
        base64::encode(&b, &mut buf);
        Ok(buf)
    }

    pub fn from_base64(input: &str) -> Result<Self> {
        let mut buf: Vec<u8> = Vec::new();
        base64::decode(input, &mut buf)?;
        Ok(bson::from_slice(&buf)?)
    }
}
