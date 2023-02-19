use super::core::{DirectionUD, Placement, SyllabicType};
use roxmltree::{Node, NodeType};
use std::str::FromStr;

#[derive(Debug)]
pub struct Note {
    pub duration: usize,
    pub notetype: String,
    pub pitch: Option<Pitch>,
    pub voice: u8,
    pub staff: u8,
    pub rest: bool,
    pub stem: Option<DirectionUD>,
    pub position: usize,
    pub chord: bool,
    pub chord_notes: Vec<Note>,
    pub lyricsAbove: Vec<Lyric>,
    pub lyricsBelow: Vec<Lyric>,
}

pub fn parse_note(el: Node, position: usize) -> Note {
    let mut duration: usize = 0;
    let mut nodetype = "";
    let mut pitch: Option<Pitch> = None;
    let mut voice: u8 = 1;
    let mut staff: u8 = 1;
    let mut rest: bool = false;
    let mut stem: Option<DirectionUD> = None;
    let mut chord: bool = false;
    let mut chord_notes: Vec<Note> = vec![];
    let mut lyricsAbove: Vec<Lyric> = vec![];
    let mut lyricsBelow: Vec<Lyric> = vec![];

    for child in el.children() {
        let child_name = child.tag_name().name();
        match child_name {
            "duration" => {
                let text = child.text();
                if let Some(x) = text {
                    duration = x.parse().unwrap_or(0);
                }
            }
            "type" => {
                let text = child.text();
                if let Some(x) = text {
                    nodetype = x;
                }
            }
            "pitch" => {
                pitch = parse_option_pitch(child);
            }
            "lyric" => {
                let lyric = parse_option_lyric(child);
                match lyric.placement {
                    Placement::Above => lyricsAbove.push(lyric),
                    Placement::Below => lyricsBelow.push(lyric),
                }
            }
            "voice" => {
                if let Some(text) = child.text() {
                    voice = text.parse().unwrap_or(1);
                };
            }
            "rest" => {
                rest = true;
            }
            "stem" => {
                if let Some(text) = child.text() {
                    stem = match text.to_lowercase().as_str() {
                        "up" => Some(DirectionUD::Up),
                        "down" => Some(DirectionUD::Down),
                        _ => None,
                    }
                }
            }
            "staff" => {
                if let Some(text) = child.text() {
                    staff = text.parse().unwrap_or(1);
                };
            }
            "chord" => {
                chord = true;
            }
            "" => {}
            _ => {
                println!("UNKNOWN note child {}", child_name);
            }
        }
    }

    Note {
        pitch,
        duration,
        notetype: nodetype.to_string(),
        voice,
        staff,
        rest,
        stem,
        position,
        chord,
        chord_notes,
        lyricsAbove,
        lyricsBelow,
    }
}

#[derive(Debug)]
pub struct Pitch {
    step: char,
    octave: u8,
}

pub fn parse_option_pitch(el: Node) -> Option<Pitch> {
    let mut step: char = 'G';
    let mut octave: u8 = 0;
    for child in el.children() {
        let child_name = child.tag_name().name();
        match child.node_type() {
            NodeType::Element => match child_name {
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
                _ => {}
            },
            _ => {}
        }
    }
    Some(Pitch { step, octave })
}

#[derive(Debug)]
pub struct Lyric {
    number: u8,
    placement: Placement,
    syllabic: SyllabicType,
    text: String,
    extend: bool,
}

pub fn parse_option_lyric(el: Node) -> Lyric {
    let mut number: u8 = 1;
    let mut syllabic: SyllabicType = SyllabicType::Single;
    let mut text: &str = "";
    let mut placement: Placement = Placement::Below;
    let mut extend: bool = false;

    for attr in el.attributes() {
        match attr.name() {
            "number" => {
                if let Ok(num) = attr.value().parse() {
                    number = num;
                }
            }
            "placement" => {
                placement = Placement::from_str(attr.value()).unwrap_or(Placement::Below);
            }
            _ => {}
        }
    }

    for child in el.children() {
        let child_name = child.tag_name().name();
        match child.node_type() {
            NodeType::Element => match child_name {
                "syllabic" => {
                    let txt_opt = child.text();
                    if let Some(txt) = txt_opt {
                        syllabic = SyllabicType::from_str(txt).unwrap_or(SyllabicType::Single);
                    }
                }
                "text" => {
                    let txt_opt = child.text();
                    if let Some(txt) = txt_opt {
                        text = txt;
                    }
                }
                "extend" => {
                    extend = true;
                }
                _ => {}
            },
            _ => {}
        }
    }

    Lyric {
        number,
        placement,
        syllabic,
        text: text.to_string(),
        extend,
    }
}
