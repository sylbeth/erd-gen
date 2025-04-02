use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::{
    color::Color,
    entries::EntityEntry,
    prelude::{AttributeKind, EntityKind},
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Entities {
    pub kind: Kind,
    pub entities: HashSet<EntityEntry>,
    #[serde(default)]
    pub color: HashMap<EntityKind, Color>,
    #[serde(default)]
    pub attributes_color: HashMap<AttributeKind, Color>,
}

impl Entities {
    pub fn to_dot(&self, dot: &mut String, is_directed: bool, is_dot: bool) {
        dot.push_str("  comment=\"Entities\"\n");
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Barker,
    Chen,
    Table,
}
