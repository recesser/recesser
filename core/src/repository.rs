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

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyPair {
    pub private_key: PrivateKey,
    pub public_key: PublicKey,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct PrivateKey(String);

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicKey {
    pub public_key: String,
    pub fingerprint: Fingerprint,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fingerprint(String);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CommitID(Option<String>);

impl Repository {
    pub fn new(name: &str, public_key: PublicKey) -> Self {
        let url = format!("git@github.com:{}.git", name);
        Self {
            name: name.to_string(),
            url,
            public_key,
            last_commit: CommitID::new(None),
        }
    }
}

impl PrivateKey {
    pub fn new(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

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

impl CommitID {
    pub fn new(s: Option<String>) -> Self {
        Self(s)
    }
}

impl fmt::Display for CommitID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.0.clone().unwrap_or_else(|| String::from("None"));
        write!(f, "{}", s)
    }
}
