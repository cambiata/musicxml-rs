use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::error::Error;
use std::fs;
fn main() -> Result<(),  Box<dyn Error>> {
    println!("Hello, world!");
    let xml = fs::read_to_string("test.xml")?;
    println!("xml:{:?}", xml);
    let mut reader = Reader::from_str(&xml);
    reader.trim_text(true);

    // let mut count = 0;
    // let mut txt = Vec::new();
    let mut buf = Vec::new();

    loop {
        // NOTE: this is the generic case when we don't know about the input BufRead.
        // when the input is a &str or a &[u8], we don't actually need to use another
        // buffer, we could directly call `reader.read_event()`
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            // exits the loop when reaching end of file
            Ok(Event::Eof) => break,

            Ok(Event::Start(e)) => {
                println!("e.name:{:?}", e.name());
                for attr in e.attributes() {
                    println!("attr:{:?}", attr);
                }
            },
            Ok(Event::Text(e)) => {
                println!("e.unescaped():{:?}", e.unescape());
            },
            Ok(Event::End(e)) => {
                println!("end:{:?}", e.name());
            }
            // Ok(Event::Start(e)) => match e.name().as_ref() {
            //     b"tag1" => println!(
            //         "attributes values: {:?}",
            //         e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()
            //     ),
            //     b"tag2" => count += 1,
            //     _ => (),
            // },
            // Ok(Event::Text(e)) => txt.push(e.unescape().unwrap().into_owned()),

            // There are several other `Event`s we do not consider here
            _ => (),
        }
        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }
    Ok(())
}
