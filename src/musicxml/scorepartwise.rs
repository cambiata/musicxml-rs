use roxmltree::{Document, Node, NodeType };
use std::error::Error;

use crate::musicxml::{part::{Part, parse_part}, scorepart::{ScorePart, parse_scorepart}};

#[derive(Debug)]
pub struct ScorePartwise {
    pub version:String,
    pub parts:Vec<Part>,
    pub partlist:Vec<ScorePart>,
}

pub fn parse_score_partwise(el: Node)->ScorePartwise {
    let mut version = "";
    let mut parts:Vec<Part> = vec![];
    let mut partlist:Vec<ScorePart> = vec![];
    for attr in el.attributes() {
        match attr.name() {
            "version" => {
                version = attr.value();    
            },
            _=>{},
        }
    }

    for child in el.children() {
        match child.node_type() {
            NodeType::Element => {
                match child.tag_name().name() {
                    "part-list" => {
                        for item in child.children() {
                            match item.node_type() {
                                NodeType::Element => {
                                    let scorepart = parse_scorepart(item);
                                    partlist.push(scorepart);
                                },
                                _=>{},
                            }
                        }
                    },
                    "part" => {
                        let part = parse_part(child);
                        parts.push(part);
                    },
                    _ => {},
                }
            },
            _ => {}
        }
    }
    ScorePartwise {version:version.to_string(), parts, partlist}
}
