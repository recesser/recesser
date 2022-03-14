use std::convert::TryFrom;

use anyhow::Error;
use serde::{Deserialize, Serialize};

use super::Pipeline;

#[derive(Deserialize, Serialize, Debug)]
pub struct TransformedPipeline {}

impl TryFrom<Pipeline> for TransformedPipeline {
    type Error = Error;

    fn try_from(value: Pipeline) -> Result<Self, Self::Error> {
        Ok(TransformedPipeline {})
    }
}
