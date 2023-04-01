use super::{core::{DirectionUD, Lyric, Pitch, Placement, SyllabicType}, lyric::parse_option_lyric};
use roxmltree::{Node, NodeType};
use std::str::FromStr;
use crate::musicxml::pitch::parse_option_pitch;


#[derive(Debug)]
pub struct Note {
    pub duration: usize,
    pub notetype: String,
    pub pitch: Option<Pitch>,
    pub voice: u8,
    pub staff: u8,
    pub rest: bool,
    pub dots: u8,
    pub stem: Option<DirectionUD>,
    pub position: usize,
    pub chord: bool,
    pub chord_notes: Vec<Note>,
    pub lyrics_above: Vec<Lyric>,
    pub lyrics_below: Vec<Lyric>,
}

pub fn parse_note(el: Node, position: usize) -> Note {
    let mut duration: usize = 0;
    let mut notetype = "";
    let mut pitch: Option<Pitch> = None;
    let mut voice: u8 = 1;
    let mut staff: u8 = 1;
    let mut rest: bool = false;
    let mut dots: u8 = 0;
    let mut stem: Option<DirectionUD> = None;
    let mut chord: bool = false;
    let mut chord_notes: Vec<Note> = vec![];
    let mut lyrics_above: Vec<Lyric> = vec![];
    let mut lyrics_below: Vec<Lyric> = vec![];

    for attr in el.attributes() {
        match attr.name() {
            "default-x" => {} // TODO?
            "default-y" => {} // TODO?
            _ => {
                println!("Unhandled note attribute: {}", attr.name());
            }
        }
    }

    for child in el.children() {
        let child_name = child.tag_name().name();
        match child_name {
            "duration" => {
                let text = child.text();
                if let Some(txt) = text {
                    duration = txt.trim().parse().unwrap_or(0);
                }
            }
            "type" => {
                let text = child.text();
                if let Some(txt) = text {
                    notetype = txt.trim();
                }
            }
            "pitch" => {
                pitch = parse_option_pitch(child);
            }
            "lyric" => {
                let lyric = parse_option_lyric(child);
                match lyric.placement {
                    Placement::Above => lyrics_above.push(lyric),
                    Placement::Below => lyrics_below.push(lyric),
                }
            }
            "voice" => {
                if let Some(text) = child.text() {
                    voice = text.trim().parse().unwrap_or(1);
                };
            }
            "rest" => {
                rest = true;
            }
            "dot" => {
                dots += 1;
            }
            "stem" => {
                if let Some(text) = child.text() {
                    stem = match text.trim().to_lowercase().as_str() {
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
                println!("UNKNOWN note child: {}", child_name);
            }
        }
    }

    Note {
        pitch,
        duration,
        notetype: notetype.to_string(),
        voice,
        staff,
        rest,
        dots,
        stem,
        position,
        chord,
        chord_notes,
        lyrics_above,
        lyrics_below: lyrics_below,
    }
}


#[cfg(test)]
mod tests_note {
    use super::parse_note;
    use roxmltree::Document;

    #[test]
    fn note() {
        let xml = r#"<note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><type>whole</type></note>"#;
        let note = parse_note(Document::parse(&xml).unwrap().root_element(), 0);
        assert_eq!(4, note.duration);
        let pitch = &note.pitch.unwrap();
        assert_eq!('C', pitch.step);
        assert_eq!(4, pitch.octave);
    }

    #[test]
    fn doubledot() {
        let xml = r#"<note><pitch><step>G</step><octave>4</octave></pitch><duration>7</duration><voice>1</voice><type>half</type><dot/><dot/><stem>up</stem></note>"#;
        let note = parse_note(Document::parse(&xml).unwrap().root_element(), 0);
        assert_eq!(2, note.dots);
    }
}
