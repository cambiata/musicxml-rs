use roxmltree::Node;

#[derive(Debug)]
pub struct Harmony {}

pub fn parse_harmony(el: Node) -> Harmony {
    for attr in el.attributes() {
        match attr.name() {
            // "number" => {
            //     number = attr.value();
            // }
            _ => {
                println!("Unhandled harmony attribute: {}", attr.name());
            }
        }
    }

    for child in el.children() {
        let child_name = child.tag_name().name();
        match child_name {
            "root" => {}
            "kind" => {}
            "bass" => {}
            "" => {}
            _ => {
                panic!("UNKNOWN harmony child: {}", child_name);
            }
        }
    }

    Harmony {}
}

/*
     <harmony>
       <root>
         <root-step>A</root-step>
         <root-alter>0</root-alter>
       </root>
       <kind text="7">dominant</kind>
       <bass>
         <bass-step>E</bass-step>
         <bass-alter>0</bass-alter>
       </bass>
     </harmony>


*/
