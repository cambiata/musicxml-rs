use roxmltree::Node;

pub fn parse_identification(el:Node) -> Identification {
    for attr in el.attributes() {
        match attr.name() {
            _ => {
                println!("UNKNOWN identification attribute: {}", attr.name());
            }
        }
    }

    for child in el.children() {
        let child_name = child.tag_name().name();
        match child_name {
            // "measure" => {
            //     let measure = parse_measure(child);
            //     measures.push(measure);
            // }
            "creator" => {}
            "encoding" => {}
            "" => {}
            _ => {
                println!("UNKNOWN identification child: {}", child_name);
            }
        }
    }

    Identification {}
        
}

#[derive(Debug)]
pub struct Identification {

}