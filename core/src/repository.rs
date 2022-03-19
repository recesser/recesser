use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NewRepository {
    pub name: String,
    pub keypair: KeyPair,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Repository {
    pub name: String,
    pub url: String,
    pub public_key: PublicKey,
    pub last_commit: CommitID,
}

impl Repository {
    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn last_commit(&self) -> &CommitID {
        &self.last_commit
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CommitID(Option<String>);

impl CommitID {
    pub fn new(s: Option<String>) -> Self {
        Self(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyPair {
    pub private_key: Vec<u8>,
    pub public_key: PublicKey,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicKey {
    pub public_key: Vec<u8>,
    pub fingerprint: Fingerprint,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fingerprint(String);

impl Fingerprint {
    pub fn new(s: String) -> Self {
        Self(s)
    }
}

impl fmt::Display for Fingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
