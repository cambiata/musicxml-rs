use crate::prelude::*;
use roxmltree::Node;

pub fn parse_identification(el: Node) -> Result<Identification> {
    for attr in el.attributes() {
        match attr.name() {
            _ => {
                println!("UNKNOWN identification attribute: {}", attr.name());
                return Err(
                    UnknownAttribute(format!("identification element: {}", attr.name())).into(),
                );
            }
        }
    }

    for child in el.children() {
        let child_name = child.tag_name().name();
        match child_name {
            "creator" => {}
            "encoding" => {}
            "" => {}
            _ => {
                println!("UNKNOWN identification child: {}", child_name);
                return Err(UnknownElement(format!("identification element: {child_name}")).into());
            }
        }
    }

    Ok(Identification {})
}

#[derive(Debug)]
pub struct Identification {}
