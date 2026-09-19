use std::io::{self, Read};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin()
        .take(256 * 1024 + 1)
        .read_to_string(&mut input)?;
    if input.len() > 256 * 1024 {
        return Err("request exceeds 256 KiB".into());
    }
    let output = cerebri_node::plan_json(&input)?;
    println!("{output}");
    Ok(())
}
