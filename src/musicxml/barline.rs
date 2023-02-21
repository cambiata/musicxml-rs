use std::str::FromStr;

use roxmltree::Node;

use super::core::{BarStyle, Location, RepeatDirection};

#[derive(Debug)]
pub struct Barline {
    pub location: Location,
    pub repeatdirection: Option<RepeatDirection>,
    pub barstyle: BarStyle,
}

pub fn parse_barline(el: Node) -> Barline {
    let mut location: Location = Location::Right;
    let mut barstyle: BarStyle = BarStyle::Standard;
    let mut repeatdirection: Option<RepeatDirection> = None;

    for attr in el.attributes() {
        match attr.name() {
            "location" => {
                location = Location::from_str(attr.value()).unwrap_or(Location::Right);
            }
            _ => {
                println!("Unhandled barline attribute: {}", attr.name());
            }
        }
    }

    for child in el.children() {
        let child_name = child.tag_name().name();
        match child_name {
            "bar-style" => {
                if let Some(txt) = child.text() {
                    barstyle = BarStyle::from_str(txt)
                        .expect(format!("Unknown barstyle: {}", txt).as_str());
                }
            }
            "repeat" => {
                for attr in child.attributes() {
                    match attr.name() {
                        "direction" => {
                            repeatdirection = RepeatDirection::from_str(attr.value()).ok();
                        }
                        _ => {
                            println!("Unhandled repeat attribute: {}", attr.name());
                        }
                    }
                }
            }
            "" => {}
            _ => {
                println!("UNKNOWN barline child: {}", child_name);
            }
        }
    }

    Barline {
        location,
        repeatdirection,
        barstyle,
    }
}
