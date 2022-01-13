use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use anyhow::Result;
use blake3::Hasher;

const CAP: usize = 1024 * 128 * 35; // Should be multiple of 128KiB to use SIMD optimizations

pub fn hash_from_disk(filepath: PathBuf) -> Result<String> {
    let mut file = File::open(filepath)?;
    let mut hasher = Hasher::new();

    loop {
        let mut buffer = [0; CAP];
        let n = file.read(&mut buffer)?;
        hasher.update_rayon(&buffer);
        if n == 0 {
            break;
        }
    }
    let hash = hasher.finalize();
    let encoded_hash = base64::encode_config(&hash.as_bytes(), base64::URL_SAFE_NO_PAD);
    Ok(encoded_hash)
}