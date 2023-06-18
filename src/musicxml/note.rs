use crate::prelude::*;

use super::{
    core::{DirectionUD, Duration, DurationType, Lyric, NotationType, Pitch, Placement},
    lyric::parse_option_lyric,
    notations::parse_notations,
};
use crate::musicxml::pitch::parse_option_pitch;
use roxmltree::Node;
use std::str::FromStr;

#[derive(Debug)]
pub struct Note {
    pub duration: Duration,
    pub notetype: DurationType,
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
    pub notations: Vec<NotationType>,
}

pub fn parse_note(el: Node, position: usize) -> Result<Note> {
    let mut duration: Duration = 0;
    let mut notetype: DurationType = DurationType::Quarter;
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
    let mut notations: Vec<NotationType> = vec![];

    for attr in el.attributes() {
        match attr.name() {
            "default-x" => {} // TODO?
            "default-y" => {} // TODO?
            _ => {
                println!("Unhandled note attribute: {}", attr.name());
                return Err(UnknownAttribute(format!("note element: {}", attr.name())).into());
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
                    notetype = DurationType::from_str(txt.trim())
                        .expect(&format!("unandled duration type: {}", txt.trim()));
                }
            }
            "pitch" => {
                pitch = parse_option_pitch(child);
            }
            "lyric" => {
                let lyric = parse_option_lyric(child)?;
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
            "notations" => {
                notations = parse_notations(child)?;
            }
            "" => {}
            _ => {
                println!("UNKNOWN note child: {}", child_name);
                return Err(UnknownElement(format!("note element: {child_name}")).into());
            }
        }
    }

    Ok(Note {
        pitch,
        duration,
        notetype,
        voice,
        staff,
        rest,
        dots,
        stem,
        position,
        chord,
        chord_notes,
        lyrics_above,
        lyrics_below,
        notations,
    })
}

#[cfg(test)]
mod tests_note {
    use super::parse_note;
    use crate::musicxml::core::Step;
    use roxmltree::Document;

    #[test]
    fn note() {
        let xml = r#"<note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><type>whole</type></note>"#;
        let note = parse_note(Document::parse(&xml).unwrap().root_element(), 0).unwrap();
        assert_eq!(4, note.duration);
        let pitch = &note.pitch.unwrap();
        assert_eq!(Step::C, pitch.step);
        assert_eq!(4, pitch.octave);
    }

    #[test]
    fn doubledot() {
        let xml = r#"<note><pitch><step>G</step><octave>4</octave></pitch><duration>7</duration><voice>1</voice><type>half</type><dot/><dot/><stem>up</stem></note>"#;
        let note = parse_note(Document::parse(&xml).unwrap().root_element(), 0).unwrap();
        assert_eq!(2, note.dots);
    }

    #[test]
    fn notations() {
        let xml = r#"<note>
        <pitch>
          <step>G</step>
          <octave>4</octave>
        </pitch>
        <duration>4</duration>
        <voice>1</voice>
        <type>quarter</type>
        <stem>up</stem>
        <staff>1</staff>
        <notations>
          <slur type="start" number="1"/>
          <tied type="stop"/>
        </notations>
        </note>"#;
        let note = parse_note(Document::parse(&xml).unwrap().root_element(), 0).unwrap();
        println!("note.notations:{:?}", note.notations);
    }
}
