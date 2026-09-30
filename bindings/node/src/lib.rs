//! Buildable Rust bridge; the JavaScript transport calls this same core.
pub fn plan_json(input: &str) -> Result<String, String> {
    cerebri_core::plan_json(input).map_err(|error| error.to_string())
}

pub fn describe_integration_json() -> Result<String, String> {
    serde_json::to_string(&cerebri_core::integration::describe())
        .map_err(|_| "integration serialization failed".into())
}

pub fn suggest_json(input: &[u8]) -> Result<String, String> {
    serde_json::to_string(&cerebri_core::integration::suggest_from_slice(input))
        .map_err(|_| "integration serialization failed".into())
}
