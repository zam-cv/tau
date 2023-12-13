use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Group {
    pub name: String,
    pub description: Option<String>,
    pub commands: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct Commands(pub BTreeMap<String, Vec<Group>>);