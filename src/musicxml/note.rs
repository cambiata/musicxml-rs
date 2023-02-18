use roxmltree::{Node, NodeType};

use super::core::DirectionUD;

#[derive(Debug)]
pub struct Note {
    pub duration: usize,
    pub notetype:String,
    pub pitch: Option<Pitch>,
    pub voice:u8,
    pub rest:bool,
    pub stem:Option<DirectionUD>,
    pub position:usize,
    pub chord:bool,
    pub chord_notes:Vec<Note>,
}

pub fn parse_note(el:Node, position:usize)-> Note {
    let mut duration : usize = 0;
    let mut nodetype = "";
    let mut pitch:Option<Pitch> = None;
    let mut voice:u8 = 1;
    let mut rest:bool = false;
    let mut stem:Option<DirectionUD> = None;
    let mut chord:bool = false;
    let mut chord_notes:Vec<Note> = vec![];

    for child in el.children() {
        let child_name = child.tag_name().name();
        match child.node_type() {
            NodeType::Element => {
                match child_name {
                    "duration" => {
                        let text = child.text();
                        if let Some(x) = text {
                            duration = x.parse().unwrap_or(0);
                        }
                    },
                    "type" => {
                        let text = child.text();
                        if let Some(x) = text {
                            nodetype = x;
                        }
                    },
                    "pitch" => {
                        pitch = parse_option_pitch(child);
                    },
                    "voice" => {
                        if let Some(text) = child.text() {
                            voice = text.parse().unwrap_or(1);
                        };
                    },
                    "rest" => {
                        rest = true;
                    }
                    "stem" => {
                        if let Some(text) = child.text() {
                            stem = match text.to_lowercase().as_str() {
                                "up" => Some(DirectionUD::Up),
                                "down" => Some(DirectionUD::Down),
                                _=> None,
                            }
                        }
                    },
                    "chord" => {
                        chord = true;
                    }
                    _=>{},
                }
            },
            _=>{},
        }
    }

    Note {
        pitch,
        duration,
        notetype: nodetype.to_string(),
        voice,
        rest,
        stem,
        position,
        chord,
        chord_notes,

    }
}
#[derive(Debug)]
pub struct Pitch {
    step:char,
    octave: u8,
}

pub fn parse_option_pitch(el: Node) -> Option<Pitch> {
    let mut step:char = 'G';
    let mut octave:u8 = 0;
    for child in el.children() {
        let child_name = child.tag_name().name();
        match child.node_type() {
            NodeType::Element => {
                match child_name {
                    "step" => {
                        let text = child.text();
                        if let Some(t) = text {
                            step = t.chars().next().unwrap();
                        }
                    }
                    "octave" => {
                        let text = child.text();
                        if let Some(x) = text {
                            if let Ok(d) = x.parse() {
                                octave = d;
                            }
                        }
                    }
                    _=> {}
                }
            },
            _ => {},
        }
    }
    Some(Pitch { step, octave})
}
