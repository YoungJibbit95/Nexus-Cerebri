fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request: cerebri_core::TemporalRequest =
        serde_json::from_str(include_str!("../../../examples/temporal-request.json"))?;
    println!(
        "{}",
        serde_json::to_string(&cerebri_core::inspect_temporal(request))?
    );
    Ok(())
}
