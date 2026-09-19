fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{}",
        cerebri_core::plan_json(include_str!("../../../examples/request.json"))?
    );
    Ok(())
}
