use crate::musicxml::{
    credit::{parse_credit, Credit},
    defaults::{parse_defaults, Defaults},
    identification::{parse_identification, Identification},
    part::{parse_part, Part},
    scorepart::{parse_scorepart, ScorePart},
    work::{parse_work, Work},
};
use crate::prelude::*;

#[derive(Debug)]
pub struct ScorePartwise {
    pub version: String,
    pub parts: Vec<Part>,
    pub partlist: Vec<ScorePart>,
    pub work: Option<Work>,
    pub identification: Option<Identification>,
    pub defaults: Option<Defaults>,
    pub credits: Vec<Credit>,
}

pub fn parse_score_partwise(el: Node) -> Result<ScorePartwise> {
    let mut version = "";
    let mut parts: Vec<Part> = vec![];
    let mut partlist: Vec<ScorePart> = vec![];
    let mut work: Option<Work> = None;
    let mut identification: Option<Identification> = None;
    let mut defaults: Option<Defaults> = None;
    let mut credits: Vec<Credit> = vec![];

    for attr in el.attributes() {
        match attr.name() {
            "version" => {
                version = attr.value();
            }
            _ => {
                return Err(
                    UnknownAttribute(format!("scorepartwise element: {}", attr.name())).into(),
                );
            }
        }
    }

    for child in el.children() {
        let child_name = child.tag_name().name();
        match child_name {
            "part-list" => {
                for item in child.children() {
                    if item.node_type() == NodeType::Element {
                        let scorepart = parse_scorepart(item)?;
                        partlist.push(scorepart);
                    }
                }
            }
            "part" => {
                let item = parse_part(child)?;
                parts.push(item);
            }
            "work" => {
                let item: Work = parse_work(child)?;
                work = Some(item);
            }
            "identification" => {
                let item: Identification = parse_identification(child)?;
                identification = Some(item);
            }
            "defaults" => {
                let item: Defaults = parse_defaults(child)?;
                defaults = Some(item);
            }
            "credit" => {
                let item: Credit = parse_credit(child)?;
                credits.push(item);
            }
            "movement-title" => {}

            "" => {}
            _ => {
                println!("Unknown score_partwise child: {}", child_name);
                return Err(UnknownElement(format!("scorepartwise element: {child_name}")).into());
            }
        }
    }
    Ok(ScorePartwise {
        version: version.to_string(),
        parts,
        partlist,
        work,
        identification,
        defaults,
        credits,
    })
}
