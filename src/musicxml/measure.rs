use roxmltree::{Node, NodeType};

use crate::musicxml::{barline::parse_barline, harmony::Harmony};

use super::{
    attributes::{parse_attributes, Attributes},
    barline::Barline,
    core::{Location, Placement},
    direction::{parse_direction, Direction},
    harmony::parse_harmony,
    note::{parse_note, Note},
};

#[derive(Debug)]
pub struct Measure {
    pub number: String,
    pub notes: Vec<Note>,
    pub directions: Vec<Direction>,
    pub attributes: Attributes,
    pub duration:usize,
}

impl Measure {
    pub fn get_voice(&self, voice_idx: u8) -> Vec<&Note> {
        let mut voice: Vec<&Note> = vec![];
        for n in &self.notes {
            if n.voice == voice_idx {
                voice.push(n)
            };
        }
        voice
    }
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
    let mut duration: usize = 0;

    // let mut parts:Vec<Part> = [];
    for attr in el.attributes() {
        match attr.name() {
            "number" => {
                number = attr.value();
            }
            "width" => {}
            _ => {
                println!("Unhandled measure attribute: {}", attr.name());
            }
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
                    duration = duration.max(curr_pos);
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
                            match item_name {
                                "duration" => {
                                    let text = item.text();
                                    if let Some(txt) = text {
                                        curr_pos -= txt.trim().parse().unwrap_or(0);
                                        duration = duration.max(curr_pos);
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
                            match item_name {
                                "duration" => {
                                    let text = item.text();
                                    if let Some(txt) = text {
                                        curr_pos += txt.trim().parse().unwrap_or(0);
                                        duration = duration.max(curr_pos);
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }

            "harmony" => {
                let harmony = parse_harmony(child);
                println!("harmony:{:?}", harmony);
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
            
            "print" => {}
            "" => {}
            _ => {
                panic!("UNKNOWN measure child: {}", child_name);
            }
        }
    }

    Measure {
        number: number.to_string(),
        notes,
        attributes,
        directions,
        duration,
    }
}

#[cfg(test)]
mod test_measure {
    use std::fs;

    use roxmltree::Document;
    use crate::musicxml::measure::parse_measure;
    
    #[test]
    fn example() {
        let xml:&str = r#"<measure number=\"1\"><attributes><divisions>1</divisions><key><fifths>0</fifths></key><time><beats>4</beats><beat-type>4</beat-type></time><clef><sign>G</sign><line>2</line></clef></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><type>whole</type></note></measure>"#;
        let item = parse_measure(Document::parse(&xml).unwrap().root_element());
        assert_eq!(1, item.notes.len());
    }

    #[test]
    fn test_voices() {
        let xml = fs::read_to_string("xml-files/measure/test-voices.xml").unwrap();
        let item = parse_measure(Document::parse(&xml).unwrap().root_element());
        assert_eq!(4, item.notes.len());
        let voice1 = item.get_voice(1);
        let voice2 = item.get_voice(2);
        assert_eq!(2, voice1.len());
        assert_eq!(2, voice2.len());
        assert_eq!(voice1[0].position, 0);
        assert_eq!(voice1[1].position, 2);
        assert_eq!(voice2[0].position, 0);
        assert_eq!(voice2[1].position, 1);
        assert_eq!(item.duration, 3);
    }

    #[test]
    fn test_lyrics1() {
        let xml = fs::read_to_string("xml-files/measure/test-lyrics1.musicxml").unwrap();
        let item = parse_measure(Document::parse(&xml).unwrap().root_element());
        for n in item.notes {
            println!("n.lyrics_below:{:?}", n.lyrics_below);
        }

    }
    
    #[test]
    fn test_lyrics_placement() {
        let xml = fs::read_to_string("xml-files/measure/test-lyrics-placement.xml").unwrap();
        let item = parse_measure(Document::parse(&xml).unwrap().root_element());
        for n in item.notes {
            println!("n.lyrics_below:{:?}", n.lyrics_below);
            println!("n.lyrics_above:{:?}", n.lyrics_above);
        }

    }
    
    #[test]
    fn test_lyrics_placement_voices() {
        let xml = fs::read_to_string("xml-files/measure/test-lyrics-placement2.xml").unwrap();
        let item = parse_measure(Document::parse(&xml).unwrap().root_element());
        for n in item.notes {
            println!("n.lyrics_below:{:?}", n.lyrics_below);
            println!("n.lyrics_above:{:?}", n.lyrics_above);
        }

    }

    #[test]
    fn test_directions_dynamics() {
        let xml = fs::read_to_string("xml-files/measure/test-directions-dynamics.xml").unwrap();
        let item = parse_measure(Document::parse(&xml).unwrap().root_element());
        for direction in item.directions {
            println!("direction:{:?}", direction);
        }
    }
}
