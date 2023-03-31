use roxmltree::{Node, NodeType};
#[derive(Debug)]
pub struct ScorePart {
    pub id: String,
    pub part_name: String,
}

pub fn parse_scorepart(el: Node) -> ScorePart {
    let mut id = "";
    let mut part_name = "";
    // let mut parts:Vec<Part> = [];
    for attr in el.attributes() {
        match attr.name() {
            "id" => {
                id = attr.value();
            }
            _ => {
                println!("UNKNOWN scorepart attribute: {}", attr.name());
            }
        }
    }

    for child in el.children() {
        let child_name = child.tag_name().name();
        match child.tag_name().name() {
            "part-name" => {
                if let Some(_part_name) = child.text() {
                    part_name = _part_name;
                }
            }
            "" => {}
            _ => {
                println!("UNKNOWN scorepart child: {}", child_name);
            }
        }
    }

    ScorePart {
        id: id.to_string(),
        part_name: part_name.to_string(),
    }
}
