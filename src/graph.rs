use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{entities::Entities, relationships::Relationships};

#[derive(Debug, Deserialize, Serialize)]
pub struct Graph {
    #[serde(default)]
    pub direction: Option<Direction>,
    #[serde(default)]
    pub size: Option<Size>,
    #[serde(default)]
    pub layout: Option<Layout>,
    pub entities: Entities,
    pub relationships: Relationships,
}

impl Graph {
    pub fn to_dot(&self) -> String {
        let mut dot = String::new();
        if let Some(direction) = &self.direction {
            dot.push_str("digraph {\n");
            dot.push_str("  rankdir=");
            dot.push_str(direction.as_str());
            dot.push_str("\n");
        } else {
            dot.push_str("graph {\n");
        }
        if let Some(size) = &self.size {
            dot.push_str("  size=");
            dot.push_str(&size.to_string());
            dot.push_str("\n");
        }
        
        if let Some(layout) = &self.layout {
            dot.push_str("  layout=");
            dot.push_str(&layout.as_str());
            dot.push_str(";\n");
        }
        dot.push_str("}\n");
        dot
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
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

#[derive(Default, Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(untagged)]
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

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(untagged)]
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
