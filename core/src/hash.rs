use std::fs::File;
use std::io::Read;
use std::path::Path;

use anyhow::Result;
use blake3::{Hash, Hasher};

const CAP: usize = 1024 * 128; // Should be multiple of 128KiB to use SIMD optimizations

pub fn hash_from_disk(filepath: &Path) -> Result<String> {
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
    Ok(encode(&hash))
}

pub fn hash(buf: &[u8]) -> String {
    let hash = blake3::hash(buf);
    encode(&hash)
}

fn encode(hash: &Hash) -> String {
    base64::encode_config(&hash.as_bytes(), base64::URL_SAFE_NO_PAD)
}

pub fn verify_integrity(buf: &[u8], checksum: &str) -> Result<()> {
    let determined_checksum = hash(buf);
    println!("Advertised checksum: {checksum}");
    println!("Determined checksum: {determined_checksum}");
    if determined_checksum.ne(checksum) {
        anyhow::bail!("Failed to verify integrity")
    }
    Ok(())
}

pub fn verify_file_integrity(filepath: &Path, checksum: &str) -> Result<String> {
    let determined_checksum = hash_from_disk(filepath)?;
    println!("Advertised checksum: {checksum}");
    println!("Determined checksum: {determined_checksum}");
    if determined_checksum.ne(checksum) {
        anyhow::bail!("Failed to verify integrity")
    }
    Ok(determined_checksum)
}
