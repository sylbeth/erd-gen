pub mod attribute;
pub mod color;
pub mod entities;
pub mod entity;
pub mod entries;
pub mod graph;
pub mod relationship;
pub mod relationships;

pub mod prelude {
    pub use super::attribute::{Attribute, Composition, Kind as AttributeKind};
    pub use super::color::Color;
    pub use super::entities::{Entities, Kind as EntitiesKind};
    pub use super::entity::{Entity, Kind as EntityKind};
    pub use super::entries::{AttributeEntry, EntityEntry, RelationshipEntry};
    pub use super::graph::{Direction, Domain, Graph, Layout, Size};
    pub use super::relationship::{Kind as RelationshipKind, Multiplicity, Relationship};
    pub use super::relationships::{Kind as RelationshipsKind, Relationships};
}
