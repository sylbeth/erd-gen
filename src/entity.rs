use serde::{Deserialize, Serialize};

use crate::attribute::Attribute;

#[derive(Debug, Deserialize, Serialize)]
pub struct Entity {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub kind: Kind,
    #[serde(default)]
    pub attributes: Vec<Attribute>,
    #[serde(default)]
    pub rank: Option<u8>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, Copy)]
pub enum Kind {
    #[default]
    Simple,
    Weak,
    Inherited,
    Associated,
}
