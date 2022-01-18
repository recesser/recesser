use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub artifact_id: String,
    pub name: Option<String>,
    pub created: Option<NaiveDateTime>,
}

impl Metadata {
    pub fn update(&mut self) {
        self.created = Some(Local::now().naive_local());
    }
}
