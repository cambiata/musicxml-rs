use roxmltree::Node;

pub fn parse_defaults(el:Node) -> Defaults {
    for attr in el.attributes() {
        match attr.name() {
            _ => {
                println!("UNKNOWN defaults attribute: {}", attr.name());
            }
        }
    }

    for child in el.children() {
        let child_name = child.tag_name().name();
        match child_name {
            "scaling" => {}
            "page-layout" => {}
            "word-font" => {}
            "lyric-font" => {}
            "" => {}
            _ => {
                println!("UNKNOWN defaults child: {}", child_name);
            }
        }
    }

    Defaults {}
        
}
#[derive(Debug)]
pub struct Defaults {

}