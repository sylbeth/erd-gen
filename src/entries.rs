use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use crate::{attribute::Attribute, entity::Entity, relationship::Relationship};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct EntityEntry(pub Entity);

impl PartialEq for EntityEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0.id == other.0.id
    }
}

impl Eq for EntityEntry {}

impl Hash for EntityEntry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.id.hash(state);
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct RelationshipEntry(pub Relationship);

impl PartialEq for RelationshipEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0.entities == other.0.entities
    }
}

impl Eq for RelationshipEntry {}

impl Hash for RelationshipEntry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.entities.hash(state);
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct AttributeEntry(pub Attribute);

impl PartialEq for AttributeEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0.name == other.0.name
    }
}

impl Eq for AttributeEntry {}

impl Hash for AttributeEntry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.name.hash(state);
    }
}
