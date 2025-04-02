use erdgen::prelude::*;

fn main() {
    let graph = Graph {
        size: Some(Size::Square(10.0)),
        direction: Some(Direction::LeftRight),
        entities: Entities { kind: EntitiesKind::Chen, entities: Vec::new() },
        relationships: Relationships { kind: RelationshipsKind::ChenArrow, relationships: Vec::new() },
        layout: Some(Layout::Dot),
    };
    println!("{:#?}", graph);
    let mut dot = String::new();
    graph.to_dot(&mut dot);
    println!("{}", dot);
}
