use roxmltree::{Node, NodeType};
#[derive(Debug)]
pub struct ScorePart {
    id: String,
}

pub fn parse_scorepart(el: Node) -> ScorePart {
    let mut id = "";
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
        let child_name = child.tag_name().name();
        match child.tag_name().name() {
            _ => {}
        }
    }

    ScorePart { id: id.to_string() }
}
