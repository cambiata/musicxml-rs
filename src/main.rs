use std::error::Error;
use std::fs;

mod musicxml;

fn main() -> Result<(), Box<dyn Error>> {
    // let xml = fs::read_to_string("music.xml")?;
    let xml = fs::read_to_string("slurs1.xml")?;
    let score = musicxml::parse(xml)?;
    let part0 = &score.parts[0];
    // for measure in &part0.measures {
    //     println!("measure.directionsAbove:{:?}", measure.directions);
    // }
    Ok(())
}
