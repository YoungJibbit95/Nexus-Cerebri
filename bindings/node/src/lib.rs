//! Buildable Rust bridge; the JavaScript transport calls this same core.
pub fn plan_json(input: &str) -> Result<String, String> {
    cerebri_core::plan_json(input).map_err(|error| error.to_string())
}
