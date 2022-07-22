mod common;

use anyhow::Result;
use recesser_schandler::workflow::Workflow;

use common::read_fixture;

#[test]
fn can_parse_template_workflow_yaml() -> Result<()> {
    let template_workflow = read_fixture("template_workflow.yml")?;
    serde_yaml::from_str::<Workflow>(&template_workflow)?;
    Ok(())
}

#[test]
fn can_parse_custom_workflow_yaml() -> Result<()> {
    let custom_workflow = read_fixture("custom_workflow.yml")?;
    serde_yaml::from_str::<Workflow>(&custom_workflow)?;
    Ok(())
}
