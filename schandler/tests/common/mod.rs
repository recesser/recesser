use std::fs;
use std::path::Path;

use anyhow::Result;

pub fn read_fixture(name: &str) -> Result<String> {
    let str_path = format!("tests/fixtures/{name}");
    Ok(fs::read_to_string(Path::new(&str_path))?)
}
