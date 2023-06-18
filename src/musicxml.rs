use crate::prelude::*;

use self::scorepartwise::ScorePartwise;

pub mod articulations;
pub mod attributes;
pub mod barline;
pub mod core;
pub mod credit;
pub mod defaults;
pub mod direction;
pub mod harmony;
pub mod identification;
pub mod lyric;
pub mod measure;
pub mod mididevice;
pub mod midiinstrument;
pub mod notations;
pub mod note;
pub mod part;
pub mod pitch;
pub mod scoreinstrument;
pub mod scorepart;
pub mod scorepartwise;
pub mod work;

//-----------------------------
pub fn parse(xml: String) -> Result<ScorePartwise> {
    let doc = Document::parse(&xml)?;
    let element = doc.root_element();
    let score_partwise = scorepartwise::parse_score_partwise(element)?;
    Ok(score_partwise)
}
