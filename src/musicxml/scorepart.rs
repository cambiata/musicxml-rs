
use roxmltree::{Node, NodeType};

use super::{
    mididevice::{parse_midi_device, MidiDevice},
    midiinstrument::{parse_midi_instrument, MidiInstrument},
    part,
    scoreinstrument::{parse_score_instrument, ScoreInstrument},
};
#[derive(Debug)]
pub struct ScorePart {
    pub id: String,
    pub part_name: String,
    pub part_abbrevieation: String,
    pub score_instrument: Option<ScoreInstrument>,
    pub midi_device: Option<MidiDevice>,
    pub midi_instrument: Option<MidiInstrument>,
}

pub fn parse_scorepart(el: Node) -> ScorePart {
    let mut id = "";
    let mut part_name = "";
    let mut part_abbreviation = "";
    let mut score_instrument: Option<ScoreInstrument> = None;
    let mut midi_device:Option<MidiDevice> = None;
    let mut midi_instrument: Option<MidiInstrument> = None;
    // let mut parts:Vec<Part> = [];
    for attr in el.attributes() {
        match attr.name() {
            "id" => {
                id = attr.value();
            }
            _ => {
                println!("UNKNOWN scorepart attribute: {}", attr.name());
            }
        }
    }

    for child in el.children() {
        let child_name = child.tag_name().name();
        match child.tag_name().name() {
            "part-name" => {
                if let Some(t) = child.text() {
                    part_name = t;
                }
            }
            "part-abbreviation" => {
                if let Some(t) = child.text() {
                    part_abbreviation = t;
                }
            }
            "score-instrument" => {
                let item: ScoreInstrument = parse_score_instrument(child);
                score_instrument = Some(item);
            }
            "midi-device" => {
                let item: MidiDevice = parse_midi_device(child);
                midi_device = Some(item);
            }
            "midi-instrument" => {
                let item: MidiInstrument = parse_midi_instrument(child);
                midi_instrument = Some(item);
            }
            "" => {}
            _ => {
                println!("UNKNOWN scorepart child: {}", child_name);
            }
        }
    }

    ScorePart {
        id: id.to_string(),
        part_name: part_name.to_string(),
        part_abbrevieation: part_abbreviation.to_string(),
        score_instrument,
        midi_device,
        midi_instrument,
    }
}
