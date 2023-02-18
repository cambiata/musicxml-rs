use std::borrow::Borrow;
use std::error::Error;
use std::fs;

mod musicxml;

use crate::musicxml::note::Note;

fn main() -> Result<(), Box<dyn Error>> {
    // let xml = fs::read_to_string("music.xml")?;
    let xml = fs::read_to_string("flox.xml")?;
    let score = musicxml::parse(xml)?;
    // println!("score:{:?}", score);

    Ok(())
}
