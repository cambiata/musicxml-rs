use std::str::FromStr;

use roxmltree::{Node, NodeType};

use crate::musicxml::core::StartStop;

use super::core::NotationType;

pub fn parse_notations(el:Node)->Vec<NotationType> {

    for attr in el.attributes() {
        match attr.name() {
            _ => {
                println!("Unhandled notations attribute: {}", attr.name());
            }
        }
    }

    let mut notation_types:Vec<NotationType> = vec![];
    for child in el.children() {
        let child_name = child.tag_name().name();
        match child.node_type() {
            NodeType::Element => match child_name {
                "slur" => {
                    let mut slur_type = StartStop::Start;
                    let mut slur_number:u8 = 0;
                    for attr in child.attributes() {
                        println!("attr:{:?}", attr);
                        match attr.name() {
                            "type" => {
                                slur_type = StartStop::from_str(attr.value()).unwrap();
                            }
                            "number" => {
                                slur_number = attr.value().parse().unwrap();
                            }
                            _=>{
                                println!("Unknown notations slur attribute:{}", attr.value());
                            }
                        }
                    }
                    notation_types.push(NotationType::Slur { s: slur_type, number: slur_number });
                }
                "tied" => {
                    let mut tie_type = StartStop::Start;
                    for attr in child.attributes() {
                        match attr.name() {
                            "type" => {
                                tie_type = StartStop::from_str(attr.value()).unwrap();
                            }
                            _=>{
                                println!("Unknown notations tied attribute:{}", attr.value());
                            }
                        }
                    }
                    notation_types.push(NotationType::Tied { s: tie_type });

                }
                _ => {
                    println!("UNKNOWN notations child: {}", child_name);
                }
            }
            _=>{}
        }
    }
    notation_types

}