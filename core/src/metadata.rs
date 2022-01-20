use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub file_content_address: String,
    pub created: Option<NaiveDateTime>,
    pub file_created: Option<NaiveDateTime>,
    pub name: Option<String>,
    pub custom: Option<serde_json::Value>,
}
