use std::path::PathBuf;

use tempfile::NamedTempFile;

pub fn tempfile() -> std::io::Result<PathBuf> {
    let named_tempfile = NamedTempFile::new()?;
    Ok(named_tempfile.path().to_path_buf())
}
