use roxmltree::Node;

pub fn parse_work(el:Node) -> Work {
    let mut title:&str = "";
    for attr in el.attributes() {
        match attr.name() {
            _ => {
                println!("UNKNOWN work attribute: {}", attr.name());
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
            "work-title" => {
                let text = child.text();
                println!("work-title {:?}", text);
                if let Some(t) = text {
                    title = t;

                }
            }
            "" => {}
            _ => {
                println!("UNKNOWN work child: {}", child_name);
            }
        }
    }

    Work {
        title:title.to_string(),
    }
    
}

#[derive(Debug)]
pub struct Work {
    title:String,
    
}