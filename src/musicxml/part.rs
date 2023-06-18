use crate::musicxml::measure::{parse_measure, Measure};
use crate::prelude::*;

#[derive(Debug)]
pub struct Part {
    pub id: String,
    pub measures: Vec<Measure>,
}

pub fn parse_part(el: Node) -> Result<Part> {
    let mut id = "";
    let mut measures: Vec<Measure> = vec![];

    for attr in el.attributes() {
        match attr.name() {
            "id" => {
                id = attr.value();
            }
            _ => {
                println!("UNKNOWN part attribute: {}", attr.name());
                return Err(UnknownAttribute(format!("part element: {}", attr.name())).into());
            }
        }
    }

    for child in el.children() {
        let child_name = child.tag_name().name();
        match child_name {
            "measure" => {
                let measure = parse_measure(child)?;
                measures.push(measure);
            }
            "" => {}
            _ => {
                println!("UNKNOWN part child: {}", child_name);
                return Err(UnknownElement(format!("part element: {child_name}")).into());
            }
        }
    }

    Ok(Part {
        id: id.to_string(),
        measures,
    })
}
