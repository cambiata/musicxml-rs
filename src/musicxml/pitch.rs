use roxmltree::{Document, Node, NodeType};

use super::core::Pitch;

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
