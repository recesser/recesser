use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(Deserialize, Serialize, Debug)]
pub struct NewUser {
    pub scope: Scope,
}

impl NewUser {
    pub fn new(scope: Scope) -> Self {
        Self { scope }
    }
}

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub scope: Scope,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, EnumString)]
pub enum Scope {
    User,
    Admin,
    Machine,
}
