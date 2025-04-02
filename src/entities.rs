use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::entries::EntityEntry;

#[derive(Debug, Deserialize, Serialize)]
pub struct Entities {
    pub kind: Kind,
    pub entities: HashSet<EntityEntry>,
}

impl Entities {
    pub fn to_dot(&self, dot: &mut String) {}
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(untagged)]
pub enum Kind {
    Barker,
    Chen,
    Table,
}
