use std::fs;
use std::path::Path;

use anyhow::{anyhow, Result};
pub use recesser_core::repository::{Fingerprint, KeyPair, PrivateKey, PublicKey};

const FILENAME: &str = "recesser-machine-key";

pub trait KeyGen {
    fn generate() -> Result<KeyPair>;
}

impl KeyGen for KeyPair {
    fn generate() -> Result<Self> {
        let dir = tempfile::tempdir()?;

        keygen_command(dir.path(), FILENAME)?;

        let priv_key_path = dir.path().join(FILENAME);
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
        .args(["-N", ""])
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

trait ReadFingerprint {
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

fn openssl_generate_key_pair() -> Result<String> {
    let keypair = openssl::pkey::PKey::generate_ed25519()?;
    let pem = keypair.private_key_to_pem_pkcs8()?;
    Ok(String::from_utf8(pem)?)
}

use ed25519_dalek::Keypair;
// use pkcs8::AlgorithmIdentifier;
// use pkcs8::LineEnding;
// use pkcs8::ObjectIdentifier;
// use pkcs8::PrivateKeyInfo;
use rand::rngs::OsRng;

// See https://datatracker.ietf.org/doc/html/rfc8410#section-3
// pub const Ed25519AlgorithmIdentifier: AlgorithmIdentifier = AlgorithmIdentifier {
//     oid: ObjectIdentifier::new("1.3.101.112"),
//     parameters: None,
// };

// fn dalek_generate_key_pair() -> Result<String> {
//     let mut csprng = OsRng {};
//     let keypair: Keypair = Keypair::generate(&mut csprng);
//     let private_key_info = PrivateKeyInfo {
//         algorithm: Ed25519AlgorithmIdentifier,
//         private_key: keypair.secret.as_bytes(),
//         public_key: None,
//     };
//     let pem = private_key_info.to_pem(LineEnding::default()).unwrap();
//     Ok(pem.to_string())
// }

use ed25519::pkcs8::EncodePrivateKey;
use ed25519::KeypairBytes;
use pkcs8::LineEnding;

fn dalek_generate_key_pair() -> Result<String> {
    let mut csprng = OsRng {};
    let keypair: Keypair = Keypair::generate(&mut csprng);
    let keypair_bytes = KeypairBytes::from_bytes(&keypair.to_bytes());
    let pem = keypair_bytes.to_pkcs8_pem(LineEnding::default()).unwrap();
    Ok(pem.to_string())
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    // #[test]
    // fn write_boringssl() -> Result<()> {
    //     let key_pair = boring_generate_key_pair()?;
    //     let pem = key_pair.private_key_to_pem_pkcs8()?;
    //     println!("{:?}", String::from_utf8(pem.clone())?);

    //     let encoded_pub_key = base64::encode(key_pair.public_key_to_der()?);
    //     println!("{:?}", encoded_pub_key);

    //     let mut file = std::fs::File::create("boring.pem")?;
    //     file.write_all(&pem)?;

    //     Ok(())
    // }

    #[test]
    fn write_openssl() -> Result<()> {
        let pem = openssl_generate_key_pair()?;
        std::fs::write("openssl.pem", pem)?;
        Ok(())
    }

    #[test]
    fn write_dalek() -> Result<()> {
        let pem = dalek_generate_key_pair()?;
        std::fs::write("dalek2.pem", pem)?;
        Ok(())
    }
}
