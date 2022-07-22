mod common;

use anyhow::Result;
use jsonschema::output::BasicOutput;
use jsonschema::JSONSchema;
use recesser_core::repository::{CommitID, Fingerprint, PublicKey, Repository};
use recesser_schandler::argo_workflows::ArgoWorkflow;
use recesser_schandler::workflow::Workflow;
use reqwest::blocking;

use common::read_fixture;

const SCHEMA_URL: &str =
    "https://raw.githubusercontent.com/argoproj/argo-workflows/v3.3.1/api/jsonschema/schema.json";

#[test]
fn produces_schema_conforming_json() -> Result<()> {
    let schema = retrieve_argo_workflows_schema()?;
    let compiled_schema = JSONSchema::compile(&schema).expect("Schema is not valid");

    let argo_workflow = mock_argo_workflow()?;
    let serialized_workflow = serde_json::to_value(&argo_workflow)?;
    println!("{}", serde_yaml::to_string(&argo_workflow)?);

    if !compiled_schema.is_valid(&serialized_workflow) {
        let result: BasicOutput = compiled_schema.apply(&serialized_workflow).basic();
        println!("{}", serde_json::to_string_pretty(&result)?);
        anyhow::bail!("Workflow does not conform to schema");
    }
    Ok(())
}

fn retrieve_argo_workflows_schema() -> Result<serde_json::Value> {
    let schema = blocking::get(SCHEMA_URL)?.json::<serde_json::Value>()?;
    Ok(schema)
}

fn mock_argo_workflow() -> Result<ArgoWorkflow> {
    let workflow: Workflow = serde_yaml::from_str(&read_fixture("template_workflow.yml")?)?;
    let repository = mock_repository();
    ArgoWorkflow::from_workflow(workflow, repository)
}

fn mock_repository() -> Repository {
    Repository {
        name: "mockRepository".into(),
        url: "notAUrl".into(),
        public_key: PublicKey {
            public_key: "notAPublicKey".into(),
            fingerprint: Fingerprint::new("notAFingerprint".into()),
        },
        last_commit: CommitID::new(None),
    }
}
