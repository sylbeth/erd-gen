use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::Write,
};

use erdgen::prelude::*;
use graphviz_rust::cmd::{CommandArg, Format};
use indexmap::IndexSet;

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
            entities: IndexSet::new(),
            color: HashMap::new(),
            attributes_color: HashMap::new(),
        },
        relationships: Relationships {
            kind: RelationshipsKind::ChenArrow,
            relationships: IndexSet::new(),
            color: HashMap::new(),
        },
    };
    let mut serial_file = File::create("graph.hjson")?;
    let mut dot_file = File::create("graph.dot")?;
    let mut dot = String::new();
    graph.to_dot(&mut dot);
    serde_hjson::to_writer(&mut serial_file, &graph)?;
    dot_file.write_all(dot.as_bytes())?;
    graphviz_rust::exec_dot(
        dot,
        vec![
            CommandArg::Output("graph.png".into()),
            CommandArg::Format(Format::Png),
        ],
    )?;
    Ok(())
}
