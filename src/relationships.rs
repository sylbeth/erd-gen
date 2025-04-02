use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::entries::RelationshipEntry;

#[derive(Debug, Serialize, Deserialize)]
pub struct Relationships {
    pub kind: Kind,
    pub relationships: HashSet<RelationshipEntry>,
}

impl Relationships {
    pub fn to_dot(&self, dot: &mut String) {}
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(untagged)]
pub enum Kind {
    Arrow,
    Bachman,
    Chen,
    ChenIsA,
    ChenArrow,
    CrowsFoot,
    UML,
}
