use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use erdgen::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let graph = Graph {
        size: Some(Size::Square(10.0)),
        direction: Some(Direction::LeftRight),
        layout: Some(Layout::Dot),
        bg_color: Some(Color::None),
        fonts: Some(Domain::All(vec![
            "Helvetica".into(),
            "Arial".into(),
            "sans-serif".into(),
        ])),
        font_color: Some(Domain::All(Color::Black)),
        entities: Entities {
            kind: EntitiesKind::Chen,
            entities: HashSet::new(),
            color: HashMap::new(),
            attributes_color: HashMap::new(),
        },
        relationships: Relationships {
            kind: RelationshipsKind::ChenArrow,
            relationships: HashSet::new(),
            color: HashMap::new(),
        },
    };
    println!("{:#?}", graph);
    let mut dot = String::new();
    graph.to_dot(&mut dot);
    println!("{}", dot);
    Ok(())
}
