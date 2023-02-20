use self::scorepartwise::ScorePartwise;
use roxmltree::{Document, Node, NodeType};
use std::error::Error;
pub mod attributes;
pub mod barline;
pub mod core;
pub mod direction;
pub mod harmony;
pub mod measure;
pub mod note;
pub mod part;
pub mod scorepart;
pub mod scorepartwise;

//-----------------------------
pub fn parse(xml: String) -> Result<ScorePartwise, Box<dyn Error>> {
    let doc = Document::parse(&xml)?;
    let element = doc.root_element();
    let score_partwise = scorepartwise::parse_score_partwise(element);
    Ok(score_partwise)
}
