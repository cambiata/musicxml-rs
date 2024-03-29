use super::core::{Lyric, Placement, SyllabicType};
use crate::prelude::*;
use std::str::FromStr;

pub fn parse_option_lyric(el: Node) -> Result<Lyric> {
    let mut number: u8 = 1;
    let mut syllabic: SyllabicType = SyllabicType::Single;
    let mut text: &str = "";
    let mut placement: Placement = Placement::Below;
    let mut extend: bool = false;

    for attr in el.attributes() {
        match attr.name() {
            "number" => {
                if let Ok(num) = attr.value().parse() {
                    number = num;
                }
            }
            "placement" => {
                placement = Placement::from_str(attr.value()).unwrap_or(Placement::Below);
            }
            _ => {
                println!("Unhandled lyric attribute: {}", attr.name());
                return Err(UnknownAttribute(format!("lyric element: {}", attr.name())).into());
            }
        }
    }

    for child in el.children() {
        let child_name = child.tag_name().name();
        match child.node_type() {
            NodeType::Element => match child_name {
                "syllabic" => {
                    let txt_opt = child.text();
                    if let Some(txt) = txt_opt {
                        syllabic = SyllabicType::from_str(txt).unwrap_or(SyllabicType::Single);
                    }
                }
                "text" => {
                    let txt_opt = child.text();
                    if let Some(txt) = txt_opt {
                        text = txt;
                    }
                }
                "extend" => {
                    extend = true;
                }
                _ => {}
            },
            _ => {
                println!("Unhandled lyric child: {}", child_name);
                return Err(UnknownElement(format!("lyric element: {child_name}")).into());
            }
        }
    }

    Ok(Lyric {
        number,
        placement,
        syllabic,
        text: text.to_string(),
        extend,
    })
}

#[cfg(test)]
mod tests {
    use super::parse_option_lyric;
    use crate::musicxml::core::{Placement, SyllabicType};
    use roxmltree::Document;
    #[test]
    fn test1() {
        let xml = r#"<lyric number="1" placement="below">
            <syllabic>begin</syllabic>
            <text>Hej</text>
        </lyric>"#;
        let item = parse_option_lyric(Document::parse(xml).unwrap().root_element()).unwrap();
        assert_eq!(Placement::Below, item.placement);
        assert_eq!(SyllabicType::Begin, item.syllabic);
        assert_eq!("Hej", item.text);
    }
}
