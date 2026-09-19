//! Synchronous planning facade shared by Rust, REST and the Node bridge.
//! No database, HTTP, provider schema, executor, clock or filesystem dependency.
use cerebri_planner::{BaselinePlanner, Planner};
pub use cerebri_planner::{PlanningRequest, PlanningResult, ValidationReport};

pub fn plan(request: PlanningRequest) -> PlanningResult {
    BaselinePlanner.plan(request)
}
pub fn validate(request: &PlanningRequest) -> ValidationReport {
    cerebri_planner::validate_request(request)
}
pub fn plan_json(input: &str) -> Result<String, serde_json::Error> {
    serde_json::to_string(&plan(serde_json::from_str(input)?))
}
pub fn validate_json(input: &str) -> Result<String, serde_json::Error> {
    serde_json::to_string(&validate(&serde_json::from_str(input)?))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn transport_neutral_json_uses_the_same_core() {
        let input = include_str!("../../../examples/request.json");
        let direct = plan(serde_json::from_str(input).unwrap());
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&plan_json(input).unwrap()).unwrap(),
            serde_json::to_value(direct).unwrap()
        );
        assert!(plan_json("{}").is_err());
    }
}
