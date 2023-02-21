use roxmltree::{Document, Node, NodeType};
use std::error::Error;

use crate::musicxml::{
    part::{parse_part, Part},
    scorepart::{parse_scorepart, ScorePart},
};

#[derive(Debug)]
pub struct ScorePartwise {
    pub version: String,
    pub parts: Vec<Part>,
    pub partlist: Vec<ScorePart>,
}

pub fn parse_score_partwise(el: Node) -> ScorePartwise {
    let mut version = "";
    let mut parts: Vec<Part> = vec![];
    let mut partlist: Vec<ScorePart> = vec![];
    for attr in el.attributes() {
        match attr.name() {
            "version" => {
                version = attr.value();
            }
            _ => {
                println!("UNKNOWN scorepartwise attribute: {}", attr.name());
            }
        }
    }

    for child in el.children() {
        let child_name = child.tag_name().name();
        match child_name {
            "part-list" => {
                for item in child.children() {
                    match item.node_type() {
                        NodeType::Element => {
                            let scorepart = parse_scorepart(item);
                            partlist.push(scorepart);
                        }
                        _ => {}
                    }
                }
            }
            "part" => {
                let part = parse_part(child);
                parts.push(part);
            }

            "" => {}
            _ => {
                println!("UNKNOWN score_partwise child: {}", child_name);
            }
        }
    }
    ScorePartwise {
        version: version.to_string(),
        parts,
        partlist,
    }
}
