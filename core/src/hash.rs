use std::fs::File;
use std::io::Read;
use std::path::Path;

use anyhow::Result;
use blake3::{Hash, Hasher};

const CAP: usize = 1024 * 128; // Should be multiple of 128KiB to leverage multi-threading and SIMD optimizations

pub fn hash_file(filepath: &Path) -> Result<[u8; 32]> {
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
    Ok(hasher.finalize().into())
}

pub fn hash_buf(buf: &[u8]) -> [u8; 32] {
    blake3::hash(buf).into()
}

/// Performs equality check in constant time.
pub fn verify_integrity(first: &[u8; 32], second: &[u8; 32]) -> Result<()> {
    let first_hash = &Hash::from(*first);
    let second_hash = &Hash::from(*second);
    if first_hash.ne(second_hash) {
        anyhow::bail!("Failed to verify integrity")
    }
    Ok(())
}
