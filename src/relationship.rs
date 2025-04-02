use serde::{Deserialize, Serialize};

use crate::attribute::Attribute;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Relationship {
    pub name: String,
    #[serde(default)]
    pub kind: Kind,
    #[serde(default)]
    pub multiplicity: Multiplicity,
    #[serde(default)]
    pub attributes: Vec<Attribute>,
    pub entities: (String, String),
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Multiplicity {
    #[default]
    One,
    OneOnlyOne,
    ZeroOrOne,
    ZeroOrMany,
    OneOrMany,
    Many,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    #[default]
    Simple,
    Strong,
    Weak,
    Identifying,
    Inheritance,
    Association,
    Dependency,
    Aggregation,
    Composition,
    Generalization,
    Implementation,
}
