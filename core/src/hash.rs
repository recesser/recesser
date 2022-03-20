use std::fs::File;
use std::io::Read;
use std::path::Path;

use anyhow::Result;

pub const DIGEST_LEN: usize = blake3::OUT_LEN;

/// Should be multiple of 128KiB to leverage multi-threading and SIMD optimizations
const BUF_LEN: usize = 1024 * 128;

pub fn hash_file(filepath: &Path) -> Result<[u8; DIGEST_LEN]> {
    let mut file = File::open(filepath)?;
    let mut hasher = blake3::Hasher::new();

    loop {
        let mut buf = [0; BUF_LEN];
        let n = file.read(&mut buf)?;
        hasher.update_rayon(&buf);
        if n == 0 {
            break;
        }
    }
    Ok(hasher.finalize().into())
}

pub fn hash_buf(buf: &[u8]) -> [u8; DIGEST_LEN] {
    blake3::hash(buf).into()
}
