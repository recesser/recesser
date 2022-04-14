use anyhow::Result;

pub fn read_fixture(name: &str) -> Result<String> {
    let str_path = format!("tests/fixtures/{name}");
    let s = std::fs::read_to_string(std::path::Path::new(&str_path))?;
    Ok(s)
}
