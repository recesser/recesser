use std::fs;
use std::path::Path;

use anyhow::{anyhow, Result};
pub use recesser_core::repository::{Fingerprint, KeyPair, PrivateKey, PublicKey};

pub trait KeyGen {
    fn generate(name: &str) -> Result<KeyPair>;
}

impl KeyGen for KeyPair {
    fn generate(name: &str) -> Result<Self> {
        let dir = tempfile::tempdir()?;
        let filename = name.replace('/', "-");

        keygen_command(dir.path(), &filename)?;

        let priv_key_path = dir.path().join(filename);
        let pub_key_path = priv_key_path.with_extension("pub");

        let private_key = PrivateKey::new(fs::read_to_string(priv_key_path)?);
        let public_key = PublicKey {
            public_key: fs::read_to_string(&pub_key_path)?,
            fingerprint: Fingerprint::read(&pub_key_path)?,
        };

        Ok(Self {
            private_key,
            public_key,
        })
    }
}

/// Generate ed25519 SSH keypair using ssh-keygen command line tool
fn keygen_command(workdir: &Path, filename: &str) -> Result<()> {
    let output = std::process::Command::new("ssh-keygen")
        .current_dir(convert_to_str(workdir)?)
        .args(["-f", filename])
        .args(["-t", "ed25519"])
        .args(["-N", "''"])
        .args(["-C", "RecesserMachineKey"])
        .output()?;
    if !output.status.success() {
        anyhow::bail!(
            "Failed to generate key. Error: {}",
            String::from_utf8(output.stderr)?
        )
    }
    Ok(())
}

pub trait ReadFingerprint {
    fn read(filepath: &Path) -> Result<Fingerprint>;
}

impl ReadFingerprint for Fingerprint {
    fn read(filepath: &Path) -> Result<Self> {
        let output = keygen_list_command(filepath)?;
        let buf = String::from_utf8(output)?;
        let fingerprint = clone_nth(&buf, 1)?;
        Ok(Self::new(fingerprint))
    }
}

fn keygen_list_command(filepath: &Path) -> Result<Vec<u8>> {
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
    Ok(output.stdout)
}

fn convert_to_str(p: &Path) -> Result<&str> {
    p.to_str()
        .ok_or_else(|| anyhow!("Failed to convert path to str"))
}

fn clone_nth(buf: &str, position: usize) -> Result<String> {
    let split_content: Vec<&str> = buf.split_whitespace().collect();
    Ok(String::from(split_content[position]))
}
