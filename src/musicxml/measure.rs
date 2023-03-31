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
}

impl Measure {
    pub fn get_voice(&self, voice_idx: u8) -> Vec<&Note> {
        // let mut voice: Vec<&Note> = self.notes.iter().filter(|n| n.voice == voice_idx).collect();
        // voice
        let mut voice: Vec<&Note> = vec![];
        for n in &self.notes {
            if (n.voice == voice_idx) {
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

    // let mut parts:Vec<Part> = [];
    for attr in el.attributes() {
        match attr.name() {
            "number" => {
                number = attr.value();
            }
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
                                    if let Some(txt) = text {
                                        curr_pos -= txt.trim().parse().unwrap_or(0);
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
                            // println!("item_name:{:?}", item_name);
                            match item_name {
                                "duration" => {
                                    let text = item.text();
                                    if let Some(txt) = text {
                                        curr_pos += txt.trim().parse().unwrap_or(0);
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
    }
}

#[cfg(test)]
mod test_measure {
    use roxmltree::Document;

    use crate::musicxml::measure::parse_measure;

    static XML:&str = "<measure number=\"1\"><attributes><divisions>1</divisions><key><fifths>0</fifths></key><time><beats>4</beats><beat-type>4</beat-type></time><clef><sign>G</sign><line>2</line></clef></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>4</duration><type>whole</type></note></measure>";

    #[test]
    fn example() {
        let item = parse_measure(Document::parse(&XML).unwrap().root_element());
        assert_eq!(1, item.notes.len());
    }

    #[test]
    fn test_backup() {
        let xml = "<measure number=\"1\" ><note ><pitch><step>G</step><octave>4</octave></pitch><duration>1</duration><voice>1</voice><type>quarter</type><stem>up</stem>
        </note><note ><pitch><step>G</step><octave>4</octave></pitch><duration>1</duration><voice>1</voice><type>quarter</type><stem>up</stem></note><note><rest/>
        <duration>2</duration><voice>1</voice><type>half</type></note><backup><duration>4</duration></backup><note ><pitch><step>E</step><alter>-1</alter><octave>4</octave>
        </pitch><duration>1</duration><voice>2</voice><type>quarter</type><accidental>flat</accidental><stem>down</stem></note><note><rest/><duration>1</duration>
        <voice>2</voice><type>quarter</type></note><note><rest/><duration>2</duration><voice>2</voice><type>half</type></note></measure>";
        let item = parse_measure(Document::parse(&xml).unwrap().root_element());
        assert_eq!(6, item.notes.len());
        let voice1 = item.get_voice(1);
        let voice2 = item.get_voice(2);
        assert_eq!(3, voice1.len());
        assert_eq!(3, voice2.len());
    }
}
