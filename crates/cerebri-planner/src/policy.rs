use crate::model::PlanningObject;
use cerebri_types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MutationKind {
    CreateEvent,
    MoveEvent,
    UpdateEvent,
    DeleteEvent,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeploymentMode {
    Local,
    Test,
    Shadow,
    Suggestion,
    Confirmation,
    LimitedAutomation,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfirmationRequirements {
    pub all_mutations: bool,
    pub deletion: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MutationPolicy {
    pub max_mutations: u32,
    pub allowed_actions: Vec<MutationKind>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicySnapshot {
    pub allow_uncertain_duration: bool,
    pub confirmation: ConfirmationRequirements,
    pub mutation: MutationPolicy,
    pub mode: DeploymentMode,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyContext {
    pub policy_set_id: PolicySetId,
    pub policy_version: Revision,
    pub snapshot: PolicySnapshot,
}
impl PolicyContext {
    pub fn permits(&self, kind: MutationKind) -> bool {
        !matches!(
            self.snapshot.mode,
            DeploymentMode::Shadow | DeploymentMode::Suggestion
        ) && self.snapshot.mutation.allowed_actions.contains(&kind)
    }
    pub fn requires_confirmation(&self, kind: MutationKind) -> bool {
        self.snapshot.confirmation.all_mutations
            || self.snapshot.mode == DeploymentMode::Confirmation
            || (kind == MutationKind::DeleteEvent && self.snapshot.confirmation.deletion)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MutationGrant {
    pub object_id: PlanningObjectId,
    pub calendar_id: CalendarId,
    pub integration_id: IntegrationId,
    pub kind: MutationKind,
}
impl MutationGrant {
    pub fn permits(&self, object: &PlanningObject, kind: MutationKind) -> bool {
        self.object_id == object.id
            && self.calendar_id == object.calendar_id
            && self.integration_id == object.integration_id
            && self.kind == kind
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlanningCapability {
    pub read: bool,
    pub plan: bool,
    pub mutations: Vec<MutationGrant>,
}
impl PlanningCapability {
    pub fn permits(&self, object: &PlanningObject, kind: MutationKind) -> bool {
        self.read && self.plan && self.mutations.iter().any(|g| g.permits(object, kind))
    }
}
/// Supplied by a trusted application boundary, never by a planning HTTP handler.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationContext {
    pub principal_id: PrincipalId,
    pub source_revision: Revision,
    pub policy: PolicyContext,
    pub planning_capability: PlanningCapability,
    pub execution_grants: Vec<MutationGrant>,
    pub confirmed_plan_ids: Vec<PlanId>,
}
