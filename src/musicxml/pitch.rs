use std::str::FromStr;

use roxmltree::{Node, NodeType};

use super::core::{Pitch, Step};

pub fn parse_option_pitch(el: Node) -> Option<Pitch> {
    let mut step: Step = Step::A;
    let mut octave: u8 = 0;
    for child in el.children() {
        let child_name = child.tag_name().name();
        match child.node_type() {
            NodeType::Element => match child_name {
                "step" => {
                    let text = child.text();
                    if let Some(t) = text {
                        step = Step::from_str(t.trim()).unwrap();
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
