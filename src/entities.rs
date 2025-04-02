use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Entities {
    pub kind: Kind,
    pub entities: Vec<Entities>,
}

impl Entities {
    pub fn to_dot(&self) -> String {
        let mut dot = String::new();

        dot
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(untagged)]
pub enum Kind {
    Barker,
    Chen,
    Table,
}

