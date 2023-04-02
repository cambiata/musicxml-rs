use std::str::FromStr;

use roxmltree::{Node};

use crate::musicxml::core::{Articulation, Placement};

use super::core::ArticulationType;
pub fn parse_articulations(el: Node) -> Vec<Articulation> {
    let mut articulations: Vec<Articulation> = vec![];

    for child in el.children() {
        let child_name = child.tag_name().name().trim();

        match child_name {
            "" => {}
            _ => {
                let mut placement: Option<Placement> = None;
                let mut typ: &str = "";

                for attr in child.attributes() {
                    match attr.name() {
                        "placement" => {
                            placement = Placement::from_str(attr.value()).ok();
                        }
                        "type" => {
                            typ = attr.value();
                        }
                        _ => {
                            println!("attr.name:{}", attr.name());
                        }
                    }
                }

                let articulation_type = ArticulationType::from_str(child_name)
                    .expect(format!("Could not find articluation for '{}'", child_name).as_str());
                articulations.push(Articulation(articulation_type, placement, typ.to_string()));
            }
        }
    }
    articulations
}

#[cfg(test)]
mod tests {
    use roxmltree::Document;

    use crate::musicxml::articulations::parse_articulations;

    #[test]
    fn articulations() {
        let xml = r#"<articulations>
              <staccato placement="below"/>
              <tenuto placement="below"/>
              <accent placement="below"/>
              <strong-accent placement="above" type="up"/>
            </articulations>"#;
        let item = parse_articulations(Document::parse(&xml).unwrap().root_element());

        println!("notations:{:?}", item);
    }
}
