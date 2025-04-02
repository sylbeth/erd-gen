use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

use crate::entries::AttributeEntry;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Attribute {
    pub name: String,
    #[serde(default)]
    pub kind: Kind,
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub composition: Composition,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    #[default]
    Simple,
    PrimaryKey,
    ForeignKey,
    Optional,
    Derived,
    Multivalued,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", untagged)]
pub enum Composition {
    Composite(IndexSet<AttributeEntry>),
    #[default]
    Simple,
}
