use anyhow::{Error, Result};
use std::fmt;
use std::path::Path;
use std::str::FromStr;

use crate::hash::{hash_buf, hash_file, verify_integrity};

const BASE64_CONFIG: base64::Config = base64::URL_SAFE_NO_PAD;
const DIGEST_LEN: usize = 32;
const HANDLE_LEN: usize = DIGEST_LEN + 2;
const BASE64_HANDLE_LEN: usize = 46;

pub struct Handle {
    version: u8,
    algorithm: u8,
    digest: [u8; DIGEST_LEN],
}

impl Handle {
    fn new(digest: [u8; DIGEST_LEN]) -> Self {
        Self {
            version: 1,
            algorithm: 1,
            digest,
        }
    }

    pub fn compute_from_buf(buf: &[u8]) -> Self {
        let digest = hash_buf(buf);
        Self::new(digest)
    }

    pub fn compute_from_file(filepath: &Path) -> Result<Self> {
        let digest = hash_file(filepath)?;
        Ok(Self::new(digest))
    }

    pub fn verify(&self, other: &Handle) -> Result<()> {
        verify_integrity(&self.digest, &other.digest)
    }

    fn serialize(&self) -> [u8; HANDLE_LEN] {
        let mut buf = [0; HANDLE_LEN];
        buf[0] = self.version;
        buf[1] = self.algorithm;
        buf[2..].copy_from_slice(&self.digest);
        buf
    }

    fn deserialize(buf: &[u8; HANDLE_LEN]) -> Self {
        let mut digest = [0; DIGEST_LEN];
        digest.copy_from_slice(&buf[2..]);
        Self {
            version: buf[0],
            algorithm: buf[1],
            digest,
        }
    }
}

impl fmt::Display for Handle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.serialize();
        let s = encode(&bytes);
        write!(f, "{s}")
    }
}

fn encode(input: &[u8; HANDLE_LEN]) -> String {
    let mut buf = String::with_capacity(BASE64_HANDLE_LEN);
    base64::encode_config_buf(input, BASE64_CONFIG, &mut buf);
    buf
}

impl FromStr for Handle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = decode(s)?;
        let handle = Handle::deserialize(&bytes);
        Ok(handle)
    }
}

fn decode(input: &str) -> Result<[u8; HANDLE_LEN]> {
    let mut buf = [0; HANDLE_LEN];
    base64::decode_config_slice(input, BASE64_CONFIG, &mut buf)?;
    Ok(buf)
}
