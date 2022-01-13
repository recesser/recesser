mod objectstorage;

use anyhow::Result;
use objectstorage::Backend;

fn main() -> Result<()> {
    let bucket = Backend::bucket("artifacts")?;
    bucket.put_object_blocking("test_file", String::from("Hello").as_bytes())?;
    let results = bucket.list_blocking(String::new(), None)?;
    println!("{:?}", results[0].contents);
    Ok(())
}
