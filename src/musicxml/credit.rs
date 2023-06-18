use crate::prelude::*;

use roxmltree::Node;

pub fn parse_credit(el: Node) -> Result<Credit> {
    for attr in el.attributes() {
        match attr.name() {
            "page" => {}
            _ => {
                println!("UNKNOWN credits attribute: {}", attr.name());
                return Err(UnknownAttribute(format!("credits element: {}", attr.name())).into());
            }
        }
    }

    for child in el.children() {
        let child_name = child.tag_name().name();
        match child_name {
            "credit-words" => {}
            "" => {}
            _ => {
                println!("UNKNOWN credits child: {}", child_name);
                return Err(UnknownElement(format!("credits element: {child_name}")).into());
            }
        }
    }

    Ok(Credit {})
}
#[derive(Debug)]
pub struct Credit {}
