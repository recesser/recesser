use anyhow::{Error, Result};
use bincode::config::{Fixint, LittleEndian, SkipFixedArrayLength};
use bincode::{Decode, Encode};
use std::fmt;
use std::path::Path;
use std::str::FromStr;

use crate::hash::{hash, hash_from_disk, verify_integrity};

const CONFIG: bincode::config::Configuration<LittleEndian, Fixint, SkipFixedArrayLength> =
    bincode::config::standard()
        .with_fixed_int_encoding()
        .skip_fixed_array_length();

#[derive(Decode, Encode)]
pub struct Handle {
    version: u8,
    algorithm: u8,
    digest: [u8; 32],
}

impl Handle {
    fn new(digest: [u8; 32]) -> Self {
        Self {
            version: 1,
            algorithm: 1,
            digest,
        }
    }

    pub fn compute(buf: &[u8]) -> Self {
        let digest = hash(buf);
        Self::new(digest)
    }

    pub fn compute_from_disk(filepath: &Path) -> Result<Self> {
        let digest = hash_from_disk(filepath)?;
        Ok(Self::new(digest))
    }

    pub fn verify(&self, buf: &[u8]) -> Result<()> {
        let digest = hash(buf);
        verify_integrity(&self.digest, &digest)
    }

    pub fn verify_from_disk(&self, filepath: &Path) -> Result<()> {
        let digest = hash_from_disk(filepath)?;
        verify_integrity(&self.digest, &digest)
    }
}

impl fmt::Display for Handle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = bincode::encode_to_vec(self, CONFIG).unwrap();
        println!("Length of serialized handle: {}", bytes.len());
        let s = encode(&bytes);
        write!(f, "{s}")
    }
}

fn encode(input: &[u8]) -> String {
    base64::encode_config(input, base64::URL_SAFE_NO_PAD)
}

impl FromStr for Handle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = decode(s)?;
        let handle: Handle = bincode::decode_from_slice(&bytes, CONFIG)?.0;
        Ok(handle)
    }
}

fn decode(input: &str) -> Result<Vec<u8>> {
    Ok(base64::decode_config(input, base64::URL_SAFE_NO_PAD)?)
}
