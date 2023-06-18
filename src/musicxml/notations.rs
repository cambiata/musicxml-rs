use super::core::NotationType;
use crate::musicxml::core::StartStop;
use roxmltree::{Node, NodeType};
use std::str::FromStr;

pub fn parse_notations(el: Node) -> Vec<NotationType> {
    for attr in el.attributes() {
        match attr.name() {
            _ => {
                println!("Unhandled notations attribute: {}", attr.name());
            }
        }
    }

    let mut notation_types: Vec<NotationType> = vec![];
    for child in el.children() {
        let child_name = child.tag_name().name();
        match child.node_type() {
            NodeType::Element => match child_name {
                "aticulations" => {
                    println!("Articulations {:?}", child.text());
                    if let Some(text) = child.text() {
                        println!("articulation text:{:?}", text);
                    }
                }
                "slur" => {
                    let mut slur_type = StartStop::Start;
                    let mut slur_number: u8 = 0;
                    for attr in child.attributes() {
                        println!("attr:{:?}", attr);
                        match attr.name() {
                            "type" => {
                                slur_type = StartStop::from_str(attr.value()).unwrap();
                            }
                            "number" => {
                                slur_number = attr.value().parse().unwrap();
                            }
                            _ => {
                                println!("Unknown notations slur attribute:{}", attr.value());
                            }
                        }
                    }
                    notation_types.push(NotationType::Slur {
                        s: slur_type,
                        number: slur_number,
                    });
                }
                "tied" => {
                    let mut tie_type = StartStop::Start;
                    for attr in child.attributes() {
                        match attr.name() {
                            "type" => {
                                tie_type = StartStop::from_str(attr.value()).unwrap();
                            }
                            _ => {
                                println!("Unknown notations tied attribute:{}", attr.value());
                            }
                        }
                    }
                    notation_types.push(NotationType::Tied { s: tie_type });
                }
                _ => {
                    println!("UNKNOWN notations child: {}", child_name);
                }
            },
            _ => {}
        }
    }
    notation_types
}

#[cfg(test)]
mod tests {
    use roxmltree::Document;

    use crate::musicxml::notations::parse_notations;

    #[test]
    fn notations() {
        let xml = r#"<notations>
            <articulations>
              <staccato placement="below"/>
              <tenuto placement="below"/>
              <accent placement="below"/>
              <strong-accent placement="above" type="up"/>
            </articulations>
          </notations>"#;
        let item = parse_notations(Document::parse(&xml).unwrap().root_element());

        println!("notations:{:?}", item);
    }
}
