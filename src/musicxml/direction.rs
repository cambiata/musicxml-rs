use std::str::FromStr;

use crate::musicxml::core::{DirectionType, WedgeType};

use super::core::{DirectionUD, Placement, SyllabicType};
use roxmltree::{Node, NodeType};

#[derive(Debug)]
pub struct Direction {
    pub position: usize,
    pub directiontypes: Vec<DirectionType>,
    pub staff: u8,
    pub placement: Option<Placement>,
    pub directive: bool,
}

pub fn parse_direction(el: Node, position: usize) -> Direction {
    println!("- parse_direction {}", position);

    let mut directiontypes: Vec<DirectionType> = vec![];
    let mut staff: u8 = 1;
    let mut placement: Option<Placement> = None;
    let mut directive: bool = false;

    for attr in el.attributes() {
        match attr.name() {
            "placement" => {
                placement = Placement::from_str(attr.value()).ok();
            }
            "directive" => {
                directive = true;
            }
            _ => {
                println!("Unhandled direction attribute: {}", attr.name());
            }
        }
    }

    for child in el.children() {
        let child_name = child.tag_name().name();
        match child_name {
            "direction-type" => {
                println!(":direction-type");
                for item in child.children() {
                    let item_name = item.tag_name().name();
                    match item_name {
                        "wedge" => {
                            let mut wedgetype: WedgeType = WedgeType::Crescendo;
                            let mut number: u8 = 1;
                            println!("- - wedge");
                            for attr in item.attributes() {
                                match attr.name() {
                                    "type" => {
                                        wedgetype = WedgeType::from_str(attr.value())
                                            .unwrap_or(WedgeType::Crescendo);
                                    }
                                    "number" => {
                                        if let Ok(num) = attr.value().parse() {
                                            number = num;
                                        }
                                    }
                                    _ => {
                                        println!("Unhandled wedge attribute: {}", attr.name());
                                    }
                                }
                            }
                            let wedge = DirectionType::Wedge { wedgetype, number };
                            println!("- - - wedge:{:?}", wedge);
                            directiontypes.push(wedge);
                        }
                        "dynamics" => {
                            println!("- - dynamics");
                            for jtem in item.children() {
                                let jtem_name = jtem.tag_name().name();
                                println!("- - - {}", jtem_name);
                                if (jtem_name.len() > 0) {
                                    directiontypes
                                        .push(DirectionType::Dynamic(jtem_name.to_string()));
                                }
                            }
                        }
                        "words" => {
                            let txt_opt = item.text();
                            if let Some(txt) = txt_opt {
                                directiontypes.push(DirectionType::Words(txt.to_string()));
                            }
                        }
                        "metronome" => {
                            println!("metronome");
                            let mut beat_unit = "quarter";
                            let mut per_minute: u8 = 0;
                            for jtem in item.children() {
                                let jtem_name = jtem.tag_name().name();
                                match jtem_name {
                                    "beat-unit" => {
                                        if let Some(n) = jtem.first_child() {
                                            beat_unit = n.text().unwrap();
                                        }
                                    }
                                    "per-minute" => {
                                        if let Some(n) = jtem.first_child() {
                                            per_minute = n.text().unwrap().parse().unwrap();
                                        }
                                    }
                                    "" => {}
                                    _ => {
                                        panic!("UNKNOWN metronome child: {}", jtem_name);
                                    }
                                }
                            }
                            directiontypes.push(DirectionType::Metronome {
                                beat_unit: beat_unit.to_string(),
                                per_minute,
                            });
                        }
                        "" => {}
                        _ => {
                            panic!("UNKNOWN direction-type: {}", item_name);
                        }
                    }
                }
            }
            "staff" => {
                let txt_opt = child.text();
                if let Some(num) = txt_opt {
                    staff = num.parse().unwrap_or(1);
                }
            }
            "" => {}
            _ => {
                panic!("UNKNOWN direction child: {}", child_name);
            }
        }
    }

    Direction {
        position,
        directive,
        directiontypes,
        staff,
        placement,
    }
}
