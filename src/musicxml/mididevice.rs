use crate::prelude::*;
use roxmltree::Node;

pub fn parse_midi_device(el: Node) -> Result<MidiDevice> {
    let mut id: &str = "";
    let mut port: u8 = 0;
    for attr in el.attributes() {
        match attr.name() {
            "id" => {
                id = attr.value();
            }
            "port" => {
                if let Ok(p) = attr.value().parse() {
                    port = p;
                }
            }
            _ => {
                println!("UNKNOWN midi_device attribute: {}", attr.name());
                return Err(
                    UnknownAttribute(format!("midi_device element: {}", attr.name())).into(),
                );
            }
        }
    }

    for child in el.children() {
        let child_name = child.tag_name().name();
        match child_name {
            "" => {}
            _ => {
                println!("UNKNOWN midi_device child: {}", child_name);
                return Err(UnknownElement(format!("midi_device element: {child_name}")).into());
            }
        }
    }

    Ok(MidiDevice {
        id: id.to_string(),
        port,
    })
}
#[derive(Debug)]
pub struct MidiDevice {
    pub id: String,
    pub port: u8,
}
