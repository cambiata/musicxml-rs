use roxmltree::{Node, NodeType};

#[derive(Debug)]
pub struct Attributes {
    pub divisions: Option<usize>,
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
            key: None,
            time: None,
            clef: None,
        }
    }
}

pub fn parse_attributes(el: Node) -> Attributes {
    let mut divisions: Option<usize> = None;
    let mut key: Option<Key> = None;
    let mut time: Option<Time> = None;
    let mut clef: Option<Clef> = None;

    for child in el.children() {
        let child_name = child.tag_name().name();

        match child.node_type() {
            NodeType::Element => match child_name {
                "divisions" => {
                    for item in child.children() {
                        match item.node_type() {
                            NodeType::Text => {
                                if let Some(x) = item.text() {
                                    divisions = Some(x.parse().unwrap());
                                }
                            }
                            _ => {}
                        }
                    }
                }
                "key" => {
                    key = parse_option_key(child);
                }
                "time" => {
                    time = parse_option_time(child);
                }
                "clef" => {
                    clef = parse_option_clef(child);
                }
                _ => {}
            },
            _ => {}
        }
    }
    Attributes {
        divisions,
        key,
        time,
        clef,
    }
}

pub fn parse_option_divisions(el: Node) -> Option<usize> {
    let mut divisions: Option<usize> = None;
    for child in el.children() {
        match child.node_type() {
            NodeType::Text => {
                if let Some(x) = child.text() {
                    if let Ok(d) = x.parse() {
                        divisions = Some(d);
                    }
                }
            }
            _ => {}
        }
    }
    divisions
}

pub fn parse_option_key(el: Node) -> Option<Key> {
    let mut key:Option<Key> = None;
    for child in el.children() {
        let child_name = child.tag_name().name();
        match child.node_type() {
            NodeType::Element => {
                match child_name {
                    "fifths" => {
                        if let Some(x) = child.text() {
                            if let Ok(d) = x.parse() {
                                key = Some(Key{fifths:d});
                            }
                        }
                    }
                    _=> {}
                }
            },
            _ => {},
        }
    }
    key
}
pub fn parse_option_time(el: Node) -> Option<Time> {
    let mut beats:u8 = 0;
    let mut beat_type:u8 = 0;
    for child in el.children() {
        let child_name = child.tag_name().name();
        match child.node_type() {
            NodeType::Element => {
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
                    _=> {}
                }
            },
            _ => {},
        }
    }

    Some(Time {
        beats,
        beat_type,
    })
}
pub fn parse_option_clef(el: Node) -> Option<Clef> {
    let mut sign:char = 'G';
    let mut line:i8 = 0;
    for child in el.children() {
        let child_name = child.tag_name().name();
        match child.node_type() {
            NodeType::Element => {
                match child_name {
                    "sign" => {
                        let text = child.text();
                        if let Some(t) = text {
                            sign = t.chars().next().unwrap();
                        }
                    }
                    "beat-type" => {
                        let text = child.text();
                        if let Some(x) = text {
                            if let Ok(d) = x.parse() {
                                line = d;
                            }
                        }
                    }
                    _=> {}
                }
            },
            _ => {},
        }
    }
    Some(Clef { sign, line})
}
