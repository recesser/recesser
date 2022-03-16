use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Repository {
    name: String,
    url: String,
    last_commit: CommitID,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitID(pub Option<String>);

impl Repository {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            url: format!("git@github.com:{name}.git"),
            last_commit: CommitID(None),
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn last_commit(&self) -> &CommitID {
        &self.last_commit
    }
}

impl PartialEq for CommitID {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
