# musicxml-rs
Might become a usable MusicXml parser, written in Rust. WIP.

```rust
fn main() -> Result<()> {
    let xml = fs::read_to_string("xml-files/hello-world.xml")?;
    let score = musicxml::parse(xml)?;
    dbg!(score);
    Ok(())
  ```
