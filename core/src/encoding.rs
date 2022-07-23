pub mod base64 {
    use anyhow::Result;

    const CONFIG: base64::Config = base64::URL_SAFE_NO_PAD;

    pub fn encode_into_buf(input: &[u8], buf: &mut String) {
        base64::encode_config_buf(input, CONFIG, buf);
    }

    pub fn decode_into_slice(input: &str, buf: &mut [u8]) -> Result<()> {
        base64::decode_config_slice(input, CONFIG, buf)?;
        Ok(())
    }

    pub fn encode(input: &[u8]) -> String {
        base64::encode_config(input, CONFIG)
    }

    pub fn decode(input: &str) -> Result<Vec<u8>> {
        Ok(base64::decode_config(input, CONFIG)?)
    }
}

pub mod hex {
    use anyhow::Result;
    use std::fmt::Write;

    pub fn encode_str(s: &str) -> Result<String> {
        let mut buf = String::new();
        for &byte in s.as_bytes() {
            write!(&mut buf, "{:x}", byte)?
        }
        Ok(buf)
    }
}
