pub mod git;
pub mod poll;
pub mod submit;
pub mod transform;

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug)]
struct Pipeline {
    name: String,
    artifact: String,
    template: Option<Template>,
    custom_workflow: Option<Value>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Template {
    language: Language,
    image: Option<String>,
    entrypoint: PathBuf,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum Language {
    Python,
    R,
}
