use std::collections::HashSet;

use erdgen::prelude::*;

fn main() {
    let graph = Graph {
        size: Some(Size::Square(10.0)),
        direction: Some(Direction::LeftRight),
        layout: Some(Layout::Dot),
        bgcolor: Some(Color::None),
        fonts: Some(Domain::All(vec![
            "Helvetica".into(),
            "Arial".into(),
            "sans-serif".into(),
        ])),
        fontcolor: Some(Domain::All(Color::Black)),
        entities: Entities {
            kind: EntitiesKind::Chen,
            entities: HashSet::new(),
        },
        relationships: Relationships {
            kind: RelationshipsKind::ChenArrow,
            relationships: HashSet::new(),
        },
    };
    println!("{:#?}", graph);
    let mut dot = String::new();
    graph.to_dot(&mut dot);
    println!("{}", dot);
}
