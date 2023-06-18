use roxmltree::{Node, NodeType};

use crate::prelude::*;

#[derive(Debug)]
pub struct Attributes {
    pub divisions: Option<usize>,
    pub staves: Option<u8>,
    pub key: Option<Key>,
    pub time: Option<Time>,
    pub clef: Option<Clef>,
}
#[derive(Debug)]
pub struct Key {
    fifths: i8,
}
#[derive(Debug)]
pub struct Time {
    beats: u8,
    beat_type: u8,
}
#[derive(Debug)]
pub struct Clef {
    sign: char,
    line: i8,
}

impl Attributes {
    pub fn empty() -> Attributes {
        Attributes {
            divisions: None,
            staves: None,
            key: None,
            time: None,
            clef: None,
        }
    }
}

pub fn parse_attributes(el: Node) -> Result<Attributes> {
    let mut divisions: Option<usize> = None;
    let mut staves: Option<u8> = None;
    let mut key: Option<Key> = None;
    let mut time: Option<Time> = None;
    let mut clef: Option<Clef> = None;

    // return Err(UnknownAttribute(format!("XXXX element: {}", attr.name())).into());

    for child in el.children() {
        let child_name = child.tag_name().name();

        match child_name {
            "divisions" => {
                for item in child.children() {
                    match item.node_type() {
                        NodeType::Text => {
                            if let Some(x) = item.text() {
                                divisions = Some(x.parse().unwrap_or(0));
                            }
                        }
                        _ => {}
                    }
                }
            }
            "staves" => {
                for item in child.children() {
                    match item.node_type() {
                        NodeType::Text => {
                            if let Some(x) = item.text() {
                                staves = Some(x.parse().unwrap_or(0));
                            }
                        }
                        _ => {}
                    }
                }
            }
            "key" => {
                key = parse_option_key(child)?;
            }
            "time" => {
                time = parse_option_time(child)?;
            }
            "clef" => {
                clef = parse_option_clef(child)?;
            }
            "" => {}
            _ => {
                println!("UNKNOWN attributes child: {}", child_name);
                return Err(UnknownElement(format!("attributes element: {child_name}")).into());
            }
        }
    }
    Ok(Attributes {
        divisions,
        staves,
        key,
        time,
        clef,
    })
}

pub fn parse_option_key(el: Node) -> Result<Option<Key>> {
    let mut key: Option<Key> = None;
    for child in el.children() {
        let child_name = child.tag_name().name();
        match child_name {
            "fifths" => {
                if let Some(x) = child.text() {
                    if let Ok(d) = x.parse() {
                        key = Some(Key { fifths: d });
                    }
                }
            }
            "mode" => {
                println!("unimplemented key child {}", child_name);
            }
            "" => {}
            _ => {
                println!("UNKNOWN key child: {}", child_name);
                return Err(UnknownElement(format!("key element: {child_name}")).into());
            }
        }
    }
    Ok(key)
}

/*
       <key number="1">
         <fifths>2</fifths>
         <mode>major</mode>
       </key>
*/

pub fn parse_option_time(el: Node) -> Result<Option<Time>> {
    let mut beats: u8 = 0;
    let mut beat_type: u8 = 0;
    for child in el.children() {
        let child_name = child.tag_name().name();
        match child_name {
            "beats" => {
                if let Some(x) = child.text() {
                    if let Ok(d) = x.parse() {
                        beats = d;
                    }
                }
            }
            "beat-type" => {
                if let Some(x) = child.text() {
                    if let Ok(d) = x.parse() {
                        beat_type = d;
                    }
                }
            }

            "" => {}
            _ => {
                println!("UNKNOWN time child: {}", child_name);
                return Err(UnknownElement(format!("time element: {child_name}")).into());
            }
        }
    }

    Ok(Some(Time { beats, beat_type }))
}

pub fn parse_option_clef(el: Node) -> Result<Option<Clef>> {
    let mut sign: char = 'G';
    let mut line: i8 = 0;
    for child in el.children() {
        let child_name = child.tag_name().name();
        match child_name {
            "sign" => {
                let text = child.text();
                if let Some(t) = text {
                    sign = t.chars().next().unwrap();
                }
            }
            "line" => {
                let text = child.text();
                if let Some(x) = text {
                    if let Ok(d) = x.parse() {
                        line = d;
                    }
                }
            }
            "" => {}
            _ => {
                println!("UNKNOWN clef child {}", child_name);
                return Err(UnknownElement(format!("clef element: {child_name}")).into());
            }
        }
    }
    Ok(Some(Clef { sign, line }))
}
