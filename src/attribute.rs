use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Attribute {
    pub name: String,
    #[serde(default)]
    pub kind: Kind,
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub composition: Composition,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(untagged)]
pub enum Kind {
    #[default]
    Simple,
    PrimaryKey,
    ForeignKey,
    Optional,
    Derived,
    Multivalued,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Composition {
    Composite(Vec<Attribute>),
    #[default]
    Simple,
}
