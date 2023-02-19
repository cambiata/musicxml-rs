use roxmltree::{Node, NodeType};

use super::measure::{parse_measure, Measure};

#[derive(Debug)]
pub struct Part {
    pub id: String,
    pub measures: Vec<Measure>,
}

pub fn parse_part(el: Node) -> Part {
    let mut id = "";
    let mut measures: Vec<Measure> = vec![];

    for attr in el.attributes() {
        match attr.name() {
            "id" => {
                id = attr.value();
            }
            _ => {}
        }
    }

    for child in el.children() {
        let child_name = child.tag_name().name();
        match child_name {
            "measure" => {
                let measure = parse_measure(child);
                measures.push(measure);
            }
            "" => {}
            _ => {
                println!("UNKNOWN part child {}", child_name);
            }
        }
    }

    Part {
        id: id.to_string(),
        measures,
    }
}
