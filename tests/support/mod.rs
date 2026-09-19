#![allow(dead_code)]
use cerebri_planner::*;
use cerebri_temporal::{Instant, TimeRange};
use cerebri_types::*;

pub fn request() -> PlanningRequest {
    serde_json::from_str(include_str!("../../examples/request.json")).unwrap()
}
pub fn range(start: &str, end: &str) -> TimeRange {
    TimeRange::new(
        start.parse::<Instant>().unwrap(),
        end.parse::<Instant>().unwrap(),
    )
    .unwrap()
}
pub fn placement(id: &str, start: &str, end: &str) -> Placement {
    Placement {
        object_id: PlanningObjectId::new(id).unwrap(),
        range: range(start, end),
    }
}
pub fn first_proposal(request: &PlanningRequest) -> ProposedPlan {
    BaselinePlanner
        .plan(request.clone())
        .candidates
        .remove(0)
        .proposed
}
pub fn authorization(request: &PlanningRequest, plan: &ActionPlan) -> AuthorizationContext {
    AuthorizationContext {
        principal_id: request.principal_id.clone(),
        source_revision: request.context.revision,
        policy: request.policy.clone(),
        planning_capability: request.planning_capability.clone(),
        execution_grants: request.planning_capability.mutations.clone(),
        confirmed_plan_ids: vec![plan.id().clone()],
    }
}
pub fn action_plan(request: &PlanningRequest) -> ActionPlan {
    first_proposal(request)
        .validate(&request.context)
        .unwrap()
        .into_action_plan()
        .unwrap()
}
