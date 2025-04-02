use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::entries::RelationshipEntry;

#[derive(Debug, Serialize, Deserialize)]
pub struct Relationships {
    pub kind: Kind,
    pub relationships: HashSet<RelationshipEntry>,
}

impl Relationships {
    fn to_dot(&self) -> String {
        let mut dot = String::new();

        dot
    }
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
