use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::entries::AttributeEntry;

#[derive(Debug, Deserialize, Serialize)]
pub struct Entity {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub kind: Kind,
    #[serde(default)]
    pub attributes: HashSet<AttributeEntry>,
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
