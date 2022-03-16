pub mod base64 {
    use anyhow::Result;

    const BASE64_CONFIG: base64::Config = base64::URL_SAFE_NO_PAD;

    pub fn encode(input: &[u8], buf: &mut String) {
        base64::encode_config_buf(input, BASE64_CONFIG, buf);
    }

    pub fn decode(input: &str, buf: &mut [u8]) -> Result<()> {
        base64::decode_config_slice(input, BASE64_CONFIG, buf)?;
        Ok(())
    }
}

pub mod serde_base64 {
    use serde::{Deserialize, Serialize};
    use serde::{Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &[u8], s: S) -> Result<S::Ok, S::Error> {
        let mut buf = String::new();
        super::base64::encode(v, &mut buf);
        String::serialize(&buf, s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
        let base64 = String::deserialize(d)?;
        let mut buf: Vec<u8> = Vec::new();
        super::base64::decode(&base64, &mut buf).map_err(serde::de::Error::custom)?;
        Ok(buf)
    }
}
