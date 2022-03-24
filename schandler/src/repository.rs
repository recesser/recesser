use std::path::{Path, PathBuf};

use anyhow::Result;
use git2::build::RepoBuilder;
use git2::{Cred, FetchOptions, RemoteCallbacks};
use recesser_core::repository::CommitID;
use tempfile::tempdir;

#[derive(Debug)]
pub struct LocalRepository {
    path: PathBuf,
    last_commit: CommitID,
}

impl LocalRepository {
    pub fn from_remote(url: &str, private_key: &str) -> Result<Self> {
        let dir = tempdir()?;
        let dirpath = dir.into_path();

        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_, username, _| {
            Cred::ssh_key_from_memory(username.unwrap(), None, private_key, None)
        });

        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);

        let mut builder = RepoBuilder::new();
        builder.fetch_options(fetch_options);

        let repo = builder.clone(url, &dirpath)?;
        let head_obj = repo.revparse_single("HEAD")?;
        let last_commit = CommitID::new(Some(head_obj.id().to_string()));

        Ok(Self {
            path: dirpath,
            last_commit,
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn last_commit(&self) -> &CommitID {
        &self.last_commit
    }
}
