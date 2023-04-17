use roxmltree::Node;

pub fn parse_score_instrument(el:Node) -> ScoreInstrument {
    let mut id:&str = "";
    let mut instrument_name:&str = "";
    for attr in el.attributes() {
        match attr.name() {
            "id" => {
                id = attr.value();
            }
            _ => {
                println!("UNKNOWN score_instrument attribute: {}", attr.name());
            }
        }
    }

    for child in el.children() {
        let child_name = child.tag_name().name();
        match child_name {
            "instrument-name" => {
                if let Some(t) = child.text() {
                    instrument_name = t;
                }
            }
            "" => {}
            _ => {
                println!("UNKNOWN score_instrument child: {}", child_name);
            }
        }
    }

    ScoreInstrument {
        id:id.to_string(),
        instrument_name: instrument_name.to_string(),
    }
        
}
#[derive(Debug)]
pub struct ScoreInstrument {
    id:String,
    instrument_name:String,
}