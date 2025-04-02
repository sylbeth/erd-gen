use serde::{Deserialize, Serialize};

use crate::{attribute::Attribute, entity::Entity};

#[derive(Debug, Deserialize, Serialize)]
pub struct Relationship {
    pub name: String,
    #[serde(default)]
    pub kind: Kind,
    #[serde(default)]
    pub multiplicity: Multiplicity,
    #[serde(default)]
    pub attributes: Vec<Attribute>,
    pub entities: (Entity, Entity),
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
#[serde(untagged)]
pub enum Multiplicity {
    #[default]
    One,
    OneOnlyOne,
    ZeroOrOne,
    ZeroOrMany,
    OneOrMany,
    Many,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(untagged)]
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
