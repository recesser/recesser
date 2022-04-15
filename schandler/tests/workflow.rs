mod common;

use anyhow::Result;
use jsonschema::output::BasicOutput;
use jsonschema::JSONSchema;
use recesser_core::repository::{CommitID, Fingerprint, PublicKey, Repository};
use recesser_schandler::argo_workflows::Workflow;
use recesser_schandler::pipeline::Pipeline;
use reqwest::blocking;

use common::read_fixture;

const SCHEMA_URL: &str =
    "https://raw.githubusercontent.com/argoproj/argo-workflows/v3.3.1/api/jsonschema/schema.json";

#[test]
fn can_transform_pipeline_into_workflow() -> Result<()> {
    let workflow = mock_workflow()?;
    println!("{}", serde_yaml::to_string(&workflow)?);
    Ok(())
}

#[test]
fn produces_valid_json() -> Result<()> {
    let schema = retrieve_argo_workflows_schema()?;
    let compiled_schema = JSONSchema::compile(&schema).expect("Schema is not valid");

    let workflow = mock_workflow()?;
    let instance = serde_json::to_value(&workflow)?;

    if !compiled_schema.is_valid(&instance) {
        let result: BasicOutput = compiled_schema.apply(&instance).basic();
        println!("{}", serde_json::to_string_pretty(&result)?);
        anyhow::bail!("Workflow does not conform to schema");
    }
    Ok(())
}

fn retrieve_argo_workflows_schema() -> Result<serde_json::Value> {
    let schema = blocking::get(SCHEMA_URL)?.json::<serde_json::Value>()?;
    Ok(schema)
}

fn mock_workflow() -> Result<Workflow> {
    let pipeline: Pipeline = serde_yaml::from_str(&read_fixture("template_pipeline.yml")?)?;
    let repository = mock_repository();
    let ssh_secret_name = "verySecretKey".into();
    Workflow::from_pipeline(pipeline, repository, ssh_secret_name)
}

fn mock_repository() -> Repository {
    Repository {
        name: "mockRepository".into(),
        url: "notAUrl".into(),
        public_key: PublicKey {
            public_key: "notAKey".into(),
            fingerprint: Fingerprint::new("notAFingerprint".into()),
        },
        last_commit: CommitID::new(None),
    }
}
