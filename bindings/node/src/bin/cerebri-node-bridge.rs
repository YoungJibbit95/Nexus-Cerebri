use cerebri_core::integration::MAX_INTEGRATION_REQUEST_BYTES;
use std::io::{self, Read};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args == ["--describe-integration"] {
        println!("{}", cerebri_node::describe_integration_json()?);
        return Ok(());
    }
    if !args.is_empty() && args != ["--suggest"] {
        return Err("unsupported bridge command".into());
    }
    let mut input = Vec::new();
    io::stdin()
        .take(MAX_INTEGRATION_REQUEST_BYTES as u64 + 1)
        .read_to_end(&mut input)?;
    let output = if args == ["--suggest"] {
        cerebri_node::suggest_json(&input)?
    } else {
        if input.len() > MAX_INTEGRATION_REQUEST_BYTES {
            return Err("request exceeds 256 KiB".into());
        }
        cerebri_node::plan_json(std::str::from_utf8(&input)?)?
    };
    println!("{output}");
    Ok(())
}
