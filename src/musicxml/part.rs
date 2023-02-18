use roxmltree::{Node, NodeType};

use super::measure::{parse_measure, Measure};

#[derive(Debug)]
pub struct Part {
    pub id: String,
    pub measures:Vec<Measure>,
}

pub fn parse_part(el: Node) -> Part {
    let mut id = "";
    let mut measures:Vec<Measure> = vec![];
    // let mut parts:Vec<Part> = [];
    for attr in el.attributes() {
        match attr.name() {
            "id" => {
                id = attr.value();
            }
            _ => {}
        }
    }

    for child in el.children() {
        match child.node_type() {
            NodeType::Element => {
                match child.tag_name().name() {
                     "measure" => {
                        let measure = parse_measure(child);
                        measures.push(measure);

                     },
                    // "part" => {

                    // },
                    _ => {}
                }
            }
            _ => {}
        }
    }

    Part {
        id: id.to_string(),
        measures,
    }
}
