use super::core::{HarmonyItem, HarmonyKind, Step};
use crate::prelude::*;
use std::str::FromStr;

#[derive(Debug)]
pub struct Harmony {
    items: Vec<HarmonyItem>,
    position: usize,
}

pub fn parse_harmony(el: Node, position: usize) -> Result<Harmony> {
    let mut items: Vec<HarmonyItem> = vec![];

    for attr in el.attributes() {
        match attr.name() {
            // "number" => {
            //     number = attr.value();
            // }
            _ => {
                println!("Unhandled harmony attribute: {}", attr.name());
                return Err(UnknownAttribute(format!("harmony element: {}", attr.name())).into());
            }
        }
    }

    for child in el.children() {
        let child_name = child.tag_name().name();
        match child_name {
            "root" => {
                let mut alter: Option<u8> = None;
                let mut step: Step = Step::A;
                child.children().for_each(|item| {
                    let item_name = item.tag_name().name();
                    match item_name {
                        "root-step" => {
                            step = Step::from_str(item.text().unwrap().trim()).unwrap();
                        }
                        "root-alter" => {
                            alter = item.text().unwrap_or("0").parse().ok();
                        }
                        "" => {}
                        _ => {
                            println!("Unhandled harmony root child: {}", item_name);
                        }
                    }
                });
                items.push(HarmonyItem::Root { step, alter });
            }
            "kind" => {
                let mut text: &str = "";
                let mut kind: HarmonyKind = HarmonyKind::Major;
                child.attributes().for_each(|attr| match attr.name() {
                    "text" => {
                        text = attr.value();
                    }
                    _ => {
                        println!("Unhandled harmony kind attribute: {}", attr.name());
                    }
                });

                child.children().for_each(|item| {
                    let item_name = item.tag_name().name().trim();
                    match item_name {
                        "text" => {
                            text = item.text().unwrap().trim();
                        }
                        "kind" => {
                            let kind_text = item.text().unwrap_or("");
                            kind = HarmonyKind::from_str(kind_text)
                                .expect(format!("Unknown harmony kind: {}", kind_text).as_str());
                        }
                        "" => {}
                        _ => {
                            println!("Unhandled harmony kind child: {}", item_name);
                        }
                    }
                });
                items.push(HarmonyItem::Kind {
                    text: text.to_string(),
                    kind,
                });
            }
            "bass" => {
                let mut step: &str = "";
                let mut alter: Option<u8> = None;
                child.children().for_each(|item| {
                    let item_name = item.tag_name().name();
                    match item_name {
                        "bass-step" => {
                            step = item.text().unwrap_or("C");
                        }
                        "bass-alter" => {
                            alter = item.text().unwrap_or("0").parse().ok();
                        }
                        "" => {}
                        _ => {
                            println!("Unhandled harmony bass child: {}", item_name);
                            // return Err(UnknownElement(format!(
                            //     "harmony bass element: {child_name}"
                            // ))
                            // .into());
                        }
                    }
                });
                items.push(HarmonyItem::Bass {
                    step: step.to_string(),
                    alter,
                });
            }
            "" => {}
            _ => {
                panic!("UNKNOWN harmony child: {}", child_name);
                return Err(UnknownElement(format!("harmony element: {child_name}")).into());
            }
        }
    }

    Ok(Harmony { items, position })
}

#[cfg(test)]
mod tests {
    use super::parse_harmony;
    use roxmltree::Document;

    #[test]
    fn example() {
        let xml = r#"<harmony>
        <root>
          <root-step>A</root-step>
          <root-alter>0</root-alter>
        </root>
        <kind text="7">dominant</kind>
        <bass>
          <bass-step>E</bass-step>
          <bass-alter>0</bass-alter>
        </bass>
      </harmony>"#;

        let item = parse_harmony(Document::parse(&xml).unwrap().root_element(), 0).unwrap();
        println!("{:?}", item);
    }
}

// rust result to option
// https://stackoverflow.com/questions/28227509/how-to-convert-a-result-to-an-option
