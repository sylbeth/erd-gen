use std::collections::HashMap;

use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

use crate::{color::Color, entries::RelationshipEntry, prelude::RelationshipKind};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Relationships {
    pub kind: Kind,
    pub relationships: IndexSet<RelationshipEntry>,
    #[serde(default)]
    pub color: HashMap<RelationshipKind, Color>,
}

impl Relationships {
    pub fn to_dot(&self, dot: &mut String, is_directed: bool) {}
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Arrow,
    Bachman,
    Chen,
    ChenIsA,
    ChenArrow,
    CrowsFoot,
    UML,
}
