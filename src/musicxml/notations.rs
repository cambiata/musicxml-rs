use crate::musicxml::articulations::parse_articulations;
use crate::musicxml::core::{Articulation, NotationType, StartStop};
use crate::prelude::*;
use roxmltree::{Node, NodeType};
use std::str::FromStr;

pub fn parse_notations(el: Node) -> Result<Vec<NotationType>> {
    for attr in el.attributes() {
        match attr.name() {
            _ => {
                println!("Unhandled notations attribute: {}", attr.name());
                return Err(UnknownAttribute(format!("notations element: {}", attr.name())).into());
            }
        }
    }

    let mut notation_types: Vec<NotationType> = vec![];
    for child in el.children() {
        let child_name = child.tag_name().name();
        match child.node_type() {
            NodeType::Element => match child_name {
                "articulations" => {
                    let articulations: Vec<Articulation> = parse_articulations(child)?;
                    for articulation in articulations.iter() {
                        notation_types.push(NotationType::Articulations(
                            articulation.0.clone(),
                            articulation.1.clone(),
                        ));
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
                    return Err(UnknownElement(format!("notations element: {child_name}")).into());
                }
            },
            _ => {}
        }
    }
    Ok(notation_types)
}

#[cfg(test)]
mod tests {
    use crate::musicxml::{
        core::{Articulation, ArticulationType, NotationType},
        notations::parse_notations,
    };
    use roxmltree::Document;

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
        let items: Vec<NotationType> =
            parse_notations(Document::parse(&xml).unwrap().root_element()).unwrap();
        assert_eq!(items.len(), 4);
        let mut iter = items.iter();
        dbg!(iter.next().unwrap());

        // assert_eq!(
        //     iter.next().unwrap(),
        //     NotationType::Articulations(
        //         crate::musicxml::core::ArticulationType::Staccato,
        //         Some(crate::musicxml::core::Placement::Below)
        //     )
        // );
    }
}
