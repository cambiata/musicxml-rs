use crate::prelude::*;

pub fn parse_midi_instrument(el: Node) -> Result<MidiInstrument> {
    let mut id: &str = "";
    let mut midi_channel: u8 = 0;
    let mut midi_program: u8 = 0;
    let mut volume: f32 = 0.;
    let mut pan: f32 = 0.;
    for attr in el.attributes() {
        match attr.name() {
            "id" => {
                id = attr.value();
            }
            _ => {
                println!("UNKNOWN midi_instrument attribute: {}", attr.name());
            }
        }
    }

    for child in el.children() {
        let child_name = child.tag_name().name();
        match child_name {
            "midi-channel" => {
                if let Some(v) = child.text() {
                    if let Ok(p) = v.parse() {
                        midi_channel = p;
                    }
                }
            }
            "midi-program" => {
                if let Some(v) = child.text() {
                    if let Ok(p) = v.parse() {
                        midi_program = p;
                    }
                }
            }
            "volume" => {
                if let Some(v) = child.text() {
                    if let Ok(p) = v.parse() {
                        volume = p;
                    }
                }
            }
            "pan" => {
                if let Some(v) = child.text() {
                    if let Ok(p) = v.parse() {
                        pan = p;
                    }
                }
            }
            "" => {}
            _ => {
                println!("UNKNOWN midi_instrument child: {}", child_name);
                return Err(
                    UnknownElement(format!("midi_instrument element: {child_name}")).into(),
                );
            }
        }
    }

    Ok(MidiInstrument {
        id: id.to_string(),
        midi_channel,
        midi_program,
        volume,
        pan,
    })
}
#[derive(Debug)]
pub struct MidiInstrument {
    pub id: String,
    pub midi_channel: u8,
    pub midi_program: u8,
    pub volume: f32,
    pub pan: f32,
}
