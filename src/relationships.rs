use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::entries::RelationshipEntry;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Relationships {
    pub kind: Kind,
    pub relationships: HashSet<RelationshipEntry>,
}

impl Relationships {
    pub fn to_dot(&self, dot: &mut String) {}
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
