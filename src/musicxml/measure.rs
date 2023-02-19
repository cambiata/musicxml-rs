use roxmltree::{Node, NodeType};

use crate::musicxml::barline::parse_barline;

use super::{
    attributes::{parse_attributes, Attributes},
    barline::Barline,
    core::{Location, Placement},
    direction::{parse_direction, Direction},
    note::{parse_note, Note},
};

#[derive(Debug)]
pub struct Measure {
    pub number: String,
    pub notes: Vec<Note>,
    pub directions: Vec<Direction>,
    pub attributes: Attributes,
}

pub fn parse_measure(el: Node) -> Measure {
    let mut number = "";
    let mut notes: Vec<Note> = vec![];
    let mut directions: Vec<Direction> = vec![];
    let mut attributes: Attributes = Attributes::empty();
    let mut curr_pos: usize = 0;
    let mut prev_note: Note;
    let mut barline_left: Barline;
    let mut barline_right: Barline;

    // let mut parts:Vec<Part> = [];
    for attr in el.attributes() {
        match attr.name() {
            "number" => {
                number = attr.value();
            }
            _ => {}
        }
    }

    for child in el.children() {
        let child_name = child.tag_name().name();
        match child_name {
            "note" => {
                let note = parse_note(child, curr_pos);
                if note.chord {
                    let len = notes.len() - 1;
                    if let Some(prev_note) = notes.get_mut(len) {
                        prev_note.chord_notes.push(note);
                    }
                } else {
                    curr_pos += note.duration;
                    notes.push(note);
                }
            }
            "direction" => {
                let direction = parse_direction(child, curr_pos);
                directions.push(direction);
            }

            "attributes" => {
                attributes = parse_attributes(child);
            }

            "backup" => {
                for item in child.children() {
                    match item.node_type() {
                        NodeType::Element => {
                            let item_name = item.tag_name().name();
                            println!("item_name:{:?}", item_name);
                            match item_name {
                                "duration" => {
                                    let text = item.text();
                                    if let Some(x) = text {
                                        curr_pos -= x.parse().unwrap_or(0);
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }

            "forward" => {
                for item in child.children() {
                    match item.node_type() {
                        NodeType::Element => {
                            let item_name = item.tag_name().name();
                            println!("item_name:{:?}", item_name);
                            match item_name {
                                "duration" => {
                                    let text = item.text();
                                    if let Some(x) = text {
                                        curr_pos += x.parse().unwrap_or(0);
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
            "sound" => {
                println!("Unhandled measure child: sound");
            }
            "barline" => {
                let barline = parse_barline(child);
                match barline.location {
                    Location::Left => barline_left = barline,
                    Location::Right => barline_right = barline,
                }
            }
            "" => {}
            _ => {
                panic!("UNKNOWN measure child {}", child_name);
            }
        }
    }

    Measure {
        number: number.to_string(),
        notes,
        attributes,
        directions,
    }
}
