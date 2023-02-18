use roxmltree::{Node, NodeType};

use super::{
    attributes::{parse_attributes, Attributes},
    note::{parse_note, Note},
};

#[derive(Debug)]
pub struct Measure {
    pub number: String,
    pub notes: Vec<Note>,
    pub attributes: Attributes,
}

pub fn parse_measure(el: Node) -> Measure {
    let mut number = "";
    let mut notes: Vec<Note> = vec![];
    let mut attributes: Attributes = Attributes::empty();
    let mut curr_pos: usize = 0;
    let mut prev_note:Note;

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
        match child.node_type() {
            NodeType::Element => {
                let el_name = child.tag_name().name();
                println!("el_name:{:?}", el_name);
                match el_name {
                    "note" => {
                        let note = parse_note(child, curr_pos);
                        println!("- note:{:?}", note);
                        if note.chord {
                            let len = notes.len()-1;
                            if let Some(prev_note) = notes.get_mut(len) {
                                prev_note.chord_notes.push(note);
                            }
                        } else {
                            curr_pos += note.duration;
                            notes.push(note);
                        }
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
                                        _ => {

                                        }
                                    }
                                }
                                _ => {
                                }
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
                                        _ => {
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }

                    _ => {}
                }
            }
            _ => {}
        }
    }
    for note in &notes {
        println!("note:{:?}-{}", note.position, note.chord_notes.len());
    }
    Measure {
        number: number.to_string(),
        notes,
        attributes,
    }
}
