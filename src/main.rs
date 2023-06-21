#![allow(dead_code, unused)]

use prelude::*;
use std::fs;

mod error;
mod musicxml;
mod prelude;

fn main() -> Result<()> {
    // let xml = fs::read_to_string("music.xml")?;
    let xml = fs::read_to_string("xml-files/hello-world.xml")?;
    let score = musicxml::parse(xml)?;
    dbg!(score);
    Ok(())
}
