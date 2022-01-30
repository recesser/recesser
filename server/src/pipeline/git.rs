use std::path::Path;

use anyhow::Result;
use git2::{build::RepoBuilder, Cred, FetchOptions, RemoteCallbacks};

const PRIVATE_KEY_PATH: &str = "/sshkey";

pub fn clone(repository: &str, dirpath: &Path) -> Result<()> {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key(
            username_from_url.unwrap(),
            None,
            Path::new(PRIVATE_KEY_PATH),
            None,
        )
    });

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    let mut builder = RepoBuilder::new();
    builder.fetch_options(fetch_options);

    builder.clone(&format!("git@github.com:{repository}.git"), dirpath)?;
    Ok(())
}
