#![allow(dead_code, unused)]

use prelude::*;
use std::fs;
mod error;
mod musicxml;
mod prelude;

fn main() -> Result<()> {
    // let xml = fs::read_to_string("music.xml")?;
    let xml = fs::read_to_string("xml-files/articulation.xml")?;
    let score = musicxml::parse(xml)?;
    let part0 = &score.parts[0];
    for score_part in &score.partlist {
        println!("{:?}", score_part);
    }
    for measure in &part0.measures {
        // println!("measure.directionsAbove:{:?}", measure.directions);
        // println!("measure:{:?}", measure);
        for note in &measure.notes {
            println!("note:{:?}", note);
        }
    }
    Ok(())
}
