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
    pub private_key: String,
    pub public_key: PublicKey,
}

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
    pub fn from_new_repository(new_repository: NewRepository) -> Self {
        let url = format!("git@github.com:{}.git", new_repository.name);
        Self {
            name: new_repository.name,
            url,
            public_key: new_repository.keypair.public_key,
            last_commit: CommitID::new(None),
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn last_commit(&self) -> &CommitID {
        &self.last_commit
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
