use std::fs;
use std::io;
use std::path::Path;

use anyhow::Result;
use brotli::CompressorWriter;

pub const BUFFER_SIZE: usize = 4096;
pub const COMPRESSION_LEVEL: u32 = 11;
pub const WINDOW_SIZE: u32 = 22;

pub fn compress_on_disk(filepath: &Path) -> Result<()> {
    let mut infile = fs::File::open(filepath)?;

    let mut outpath = filepath.to_owned();
    outpath.set_extension("brotli");
    let mut outfile = fs::File::create(&outpath)?;

    let mut compressor_writer =
        CompressorWriter::new(&mut outfile, BUFFER_SIZE, COMPRESSION_LEVEL, WINDOW_SIZE);
    io::copy(&mut infile, &mut compressor_writer)?;

    Ok(())
}
