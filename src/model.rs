use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Squad {
    pub squad_name: String,
    pub home_town: String,
    pub formed: i64,
    pub secret_base: String,
    pub active: bool,
    pub members: Vec<Member>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    pub name: String,
    pub age: i64,
    pub secret_identity: String,
    pub powers: Vec<String>,
}