use serde::{Deserialize, Serialize};

use crate::handle::Handle;

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub object_handle: Handle,
    pub custom: Option<serde_json::Value>,
}
