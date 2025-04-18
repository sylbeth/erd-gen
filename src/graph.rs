use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{color::Color, entities::Entities, relationships::Relationships};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Graph {
    #[serde(default)]
    pub direction: Option<Direction>,
    #[serde(default)]
    pub size: Option<Size>,
    #[serde(default)]
    pub layout: Option<Layout>,
    #[serde(default)]
    pub bg_color: Option<Color>,
    #[serde(default)]
    pub fonts: Option<Domain<Vec<String>>>,
    #[serde(default)]
    pub font_color: Option<Domain<Color>>,
    pub entities: Entities,
    pub relationships: Relationships,
}

impl Graph {
    pub fn to_dot(&self, dot: &mut String) {
        let is_directed = if let Some(direction) = &self.direction {
            dot.push_str("digraph {\n");
            dot.push_str("  rankdir=");
            dot.push_str(direction.as_str());
            dot.push_str(";\n");
            true
        } else {
            dot.push_str("graph {\n");
            false
        };

        if let Some(size) = &self.size {
            dot.push_str("  size=");
            dot.push_str(&size.to_string());
            dot.push_str(";\n");
        }

        if let Some(bg_color) = &self.bg_color {
            dot.push_str("  bgcolor=");
            dot.push_str(&bg_color.as_str());
            dot.push_str(";\n");
        }

        let is_dot = if let Some(layout) = &self.layout {
            dot.push_str("  layout=");
            dot.push_str(&layout.as_str());
            dot.push_str(";\n");
            layout == &Layout::Dot
        } else {
            is_directed
        };

        if let Some(fonts) = &self.fonts {
            let (graph_fonts, node_fonts, edge_fonts): (&Vec<String>, &Vec<String>, &Vec<String>);
            match fonts {
                Domain::All(fonts) => {
                    graph_fonts = fonts;
                    node_fonts = fonts;
                    edge_fonts = fonts;
                }
                Domain::Each { graph, node, edge } => {
                    graph_fonts = graph;
                    node_fonts = node;
                    edge_fonts = edge;
                }
            }
            dot.push_str("  fontname=\"");
            dot.push_str(&graph_fonts.join(","));
            dot.push_str(";\"\n  node [fontname=\"");
            dot.push_str(&node_fonts.join(","));
            dot.push_str("\"];\n  edge [fontname=\"");
            dot.push_str(&edge_fonts.join(","));
            dot.push_str("\"];\n");
        }

        if let Some(font_color) = &self.font_color {
            let (graph_fc, node_fc, edge_fc): (&Color, &Color, &Color);
            match font_color {
                Domain::All(font_color) => {
                    graph_fc = font_color;
                    node_fc = font_color;
                    edge_fc = font_color;
                }
                Domain::Each { graph, node, edge } => {
                    graph_fc = graph;
                    node_fc = node;
                    edge_fc = edge;
                }
            }
            dot.push_str("  fontcolor=");
            dot.push_str(&graph_fc.as_str());
            dot.push_str(";\n  node [fontcolor=");
            dot.push_str(&node_fc.as_str());
            dot.push_str("];\n  edge [fontcolor=");
            dot.push_str(&edge_fc.as_str());
            dot.push_str("];\n");
        }

        dot.push('\n');
        self.entities.to_dot(dot, is_directed, is_dot);
        dot.push('\n');
        self.relationships.to_dot(dot, is_directed);
        dot.push_str("\n}\n");
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", untagged)]
pub enum Size {
    Square(f64),
    Rectangle(f64, f64),
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Square(side) => write!(f, "{side}"),
            Self::Rectangle(width, height) => write!(f, "\"{width},{height}\""),
        }
    }
}

#[derive(Default, Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    #[default]
    TopBottom,
    BottomTop,
    LeftRight,
    RightLeft,
}

impl Direction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Direction::BottomTop => "BT",
            Direction::LeftRight => "LR",
            Direction::RightLeft => "RL",
            Direction::TopBottom => "TB",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Layout {
    Dot,
    Neato,
    FDP,
    SFDP,
    Circo,
    TwoPi,
    Nop,
    Nop2,
    Osage,
    Patchwork,
}

impl Layout {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Circo => "circo",
            Self::Dot => "dot",
            Self::FDP => "fdp",
            Self::Neato => "neato",
            Self::Nop => "nop",
            Self::Nop2 => "nop2",
            Self::Osage => "osage",
            Self::Patchwork => "patchwork",
            Self::SFDP => "sfdp",
            Self::TwoPi => "twopi",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", untagged)]
pub enum Domain<T> {
    All(T),
    Each { graph: T, node: T, edge: T },
}
