use std::error::Error;

use crate::prelude::*;
use roxmltree::Node;

pub fn parse_work(el: Node) -> Result<Work> {
    let mut title: &str = "";
    for attr in el.attributes() {
        match attr.name() {
            _ => {
                println!("UNKNOWN work attribute: {}", attr.name());
                return Err(UnknownAttribute(format!("work element: {}", attr.name())).into());
            }
        }
    }

    for child in el.children() {
        let child_name = child.tag_name().name();
        match child_name {
            "work-title" => {
                title = child.text().ok_or(TextfieldEmpty(format!(
                    "works field \"{}\" is empty",
                    child_name
                )))?;
            }
            "" => {}
            _ => {
                println!("UNKNOWN work child: {}", child_name);
                return Err(UnknownElement(format!("work element: {child_name}")).into());
            }
        }
    }

    Ok(Work {
        title: title.to_string(),
    })
}

#[derive(Debug)]
pub struct Work {
    title: String,
}
