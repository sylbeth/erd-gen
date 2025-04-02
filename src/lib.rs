pub mod relationships;
pub mod entity;
pub mod graph;
pub mod entities;
pub mod relationship;
pub mod attribute;

pub mod prelude {
    pub use super::attribute::{Attribute, Composition, Kind as AttributeKind};
    pub use super::entities::{Kind as EntitiesKind, Entities};
    pub use super::entity::{Entity, Kind as EntityKind};
    pub use super::graph::{Graph, Size, Direction};
    pub use super::relationship::{Kind as RelationshipKind, Multiplicity, Relationship};
    pub use super::relationships::{Kind as RelationshipsKind, Relationships};
}
