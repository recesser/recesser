use std::ops::Deref;
use std::path::{Path, PathBuf};

use anyhow::Result;
use git2::build::RepoBuilder;
use git2::{Cred, FetchOptions, RemoteCallbacks};
use tempfile::tempdir;

pub struct LocalRepository {
    path: PathBuf,
}

impl LocalRepository {
    pub fn clone(repository: &str, private_key_path: &Path) -> Result<Self> {
        let dir = tempdir()?;
        let dirpath = dir.into_path();

        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            Cred::ssh_key(username_from_url.unwrap(), None, private_key_path, None)
        });

        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);

        let mut builder = RepoBuilder::new();
        builder.fetch_options(fetch_options);

        builder.clone(&format!("git@github.com:{repository}.git"), &dirpath)?;
        Ok(Self { path: dirpath })
    }
}

impl Deref for LocalRepository {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}
