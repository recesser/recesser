mod common;

use anyhow::Result;
use recesser_schandler::pipeline::Pipeline;

use common::read_fixture;

#[test]
fn can_parse_template_pipeline_yaml() -> Result<()> {
    let template_pipeline = read_fixture("template_pipeline.yml")?;
    serde_yaml::from_str::<Pipeline>(&template_pipeline)?;
    Ok(())
}

#[test]
fn can_parse_custom_pipeline_yaml() -> Result<()> {
    let custom_pipeline = read_fixture("custom_pipeline.yml")?;
    serde_yaml::from_str::<Pipeline>(&custom_pipeline)?;
    Ok(())
}
