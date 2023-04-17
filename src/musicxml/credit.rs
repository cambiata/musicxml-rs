use roxmltree::Node;

pub fn parse_credit(el:Node) -> Credit {
    for attr in el.attributes() {
        match attr.name() {
            "page" => {}
            _ => {
                println!("UNKNOWN credits attribute: {}", attr.name());
            }
        }
    }

    for child in el.children() {
        let child_name = child.tag_name().name();
        match child_name {
            "credit-words" => {}
            "" => {}
            _ => {
                println!("UNKNOWN credits child: {}", child_name);
            }
        }
    }

    Credit {}
        
}
#[derive(Debug)]
pub struct Credit {

}