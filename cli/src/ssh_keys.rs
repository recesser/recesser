use std::fmt;
use std::fs;
use std::path::Path;

use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct KeyPair {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
    pub fingerprint: Fingerprint,
}

#[derive(Debug)]
pub struct Fingerprint(String);

impl KeyPair {
    pub fn generate(repo: &str) -> Result<Self> {
        let dir = tempfile::tempdir()?;
        let filename = repo.replace('/', "-");

        keygen(dir.path(), &filename)?;

        let priv_key_path = dir.path().join(filename);
        let pub_key_path = priv_key_path.with_extension("pub");

        let private_key = fs::read(priv_key_path)?;
        let public_key = fs::read(&pub_key_path)?;
        let fingerprint = fingerprint(&pub_key_path)?;

        Ok(Self {
            private_key,
            public_key,
            fingerprint,
        })
    }
}

fn keygen(workdir: &Path, filename: &str) -> Result<()> {
    let output = std::process::Command::new("ssh-keygen")
        .current_dir(convert_to_str(workdir)?)
        .args(["-f", filename])
        .args(["-t", "ed25519"])
        .args(["-N", "''"])
        .args(["-C", "''"])
        .output()?;
    if !output.status.success() {
        anyhow::bail!(
            "Failed to generate key. Error: {}",
            String::from_utf8(output.stderr)?
        )
    }
    Ok(())
}

fn fingerprint(filepath: &Path) -> Result<Fingerprint> {
    let output = std::process::Command::new("ssh-keygen")
        .args(["-f", convert_to_str(filepath)?])
        .arg("-l")
        .args(["-E", "sha256"])
        .output()?;
    if !output.status.success() {
        anyhow::bail!(
            "Failed to read fingerprint. Error: {}",
            String::from_utf8(output.stderr)?
        )
    }

    let buf = String::from_utf8(output.stdout)?;
    let fingerprint = clone_nth(&buf, 1)?;
    Ok(Fingerprint(fingerprint))
}

fn convert_to_str(p: &Path) -> Result<&str> {
    p.to_str().ok_or(anyhow!("Failed to convert path to str"))
}

fn clone_nth(buf: &str, position: usize) -> Result<String> {
    let split_content: Vec<&str> = buf.split_whitespace().collect();
    Ok(String::from(split_content[position]))
}

impl fmt::Display for Fingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
