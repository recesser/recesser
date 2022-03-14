use std::path::Path;

use anyhow::Result;
use git2::build::RepoBuilder;
use git2::{Cred, FetchOptions, RemoteCallbacks};

pub fn clone(repository: &str, dirpath: &Path, private_key_path: &Path) -> Result<()> {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key(username_from_url.unwrap(), None, private_key_path, None)
    });

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    let mut builder = RepoBuilder::new();
    builder.fetch_options(fetch_options);

    builder.clone(&format!("git@github.com:{repository}.git"), dirpath)?;
    Ok(())
}
