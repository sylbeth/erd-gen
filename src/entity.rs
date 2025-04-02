use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

use crate::entries::AttributeEntry;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Entity {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub kind: Kind,
    #[serde(default)]
    pub attributes: IndexSet<AttributeEntry>,
    #[serde(default)]
    pub rank: Option<u8>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    #[default]
    Simple,
    Weak,
    Inherited,
    Associated,
}
