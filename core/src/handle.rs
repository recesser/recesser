use anyhow::{Error, Result};
use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};

use std::fmt;
use std::path::Path;
use std::str::FromStr;

use crate::encoding;
use crate::hash::{hash_buf, hash_file, DIGEST_LEN};

const HANDLE_LEN: usize = DIGEST_LEN + 2;
const BASE64_HANDLE_LEN: usize = 46;

#[derive(PartialEq, Clone)]
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
        if self.ne(other) {
            anyhow::bail!("Failed to verify integrity")
        }
        Ok(())
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

impl fmt::Debug for Handle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl fmt::Display for Handle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.serialize();
        let mut buf = String::with_capacity(BASE64_HANDLE_LEN);
        encoding::base64::encode_into_buf(&bytes, &mut buf);
        write!(f, "{buf}")
    }
}

impl FromStr for Handle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut buf = [0; HANDLE_LEN];
        encoding::base64::decode_into_slice(s, &mut buf)?;
        let handle = Handle::deserialize(&buf);
        Ok(handle)
    }
}

impl Serialize for Handle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = self.to_string();
        serializer.serialize_str(&s)
    }
}

struct HandleVisitor;

impl<'de> Visitor<'de> for HandleVisitor {
    type Value = Handle;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Base64 encoded handle")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(FromStr::from_str(value).unwrap())
    }
}

impl<'de> Deserialize<'de> for Handle {
    fn deserialize<D>(deserializer: D) -> Result<Handle, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(HandleVisitor)
    }
}
