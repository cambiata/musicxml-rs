use self::scorepartwise::ScorePartwise;
use roxmltree::{Document, Node, NodeType};
use std::error::Error;
mod attributes;
mod core;
mod measure;
pub mod note;
mod part;
mod scorepart;
mod scorepartwise;

//-----------------------------
pub fn parse(xml: String) -> Result<ScorePartwise, Box<dyn Error>> {
    let doc = Document::parse(&xml)?;
    let element = doc.root_element();
    let score_partwise = scorepartwise::parse_score_partwise(element);
    Ok(score_partwise)
}
