use anyhow::{Error, Result};
use bincode::config::{Fixint, LittleEndian, SkipFixedArrayLength};
use bincode::{Decode, Encode};
use std::fmt;
use std::path::Path;
use std::str::FromStr;

use crate::hash::{hash_buf, hash_file, verify_integrity};

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
