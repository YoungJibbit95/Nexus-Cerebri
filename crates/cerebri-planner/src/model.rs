use crate::policy::{PlanningCapability, PolicyContext};
use cerebri_constraints::{ConstraintSpec, Fact};
use cerebri_preferences::PreferenceProfile;
use cerebri_semantics::SemanticVector;
use cerebri_temporal::{Deadline, Duration, Instant, TimeRange, TimeZoneId};
use cerebri_types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Operation {
    Create,
    Move,
    Update,
    Cancel,
    FindSlot,
    Reschedule,
    Optimize,
    Plan,
    Analyze,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskDetails {
    pub splittable: bool,
    pub interruptible: bool,
    pub minimum_chunk: Option<Duration>,
    pub preferred_chunk: Option<Duration>,
    pub maximum_chunk_count: Option<u32>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "details", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlanningObjectKind {
    Event,
    Task(TaskDetails),
    Deadline(Deadline),
    Availability,
    Resource,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlanningObject {
    pub id: PlanningObjectId,
    /// None denotes a prospective object; Some denotes existing external state.
    pub revision: Option<Revision>,
    pub kind: PlanningObjectKind,
    pub calendar_id: CalendarId,
    pub integration_id: IntegrationId,
    pub resource_ids: Vec<ResourceId>,
    pub time: EvidenceField<TimeRange>,
    pub timezone: TimeZoneId,
    pub semantics: Option<SemanticVector>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlanningScope {
    pub time_range: TimeRange,
    pub calendar_ids: Option<Vec<CalendarId>>,
    pub object_ids: Option<Vec<PlanningObjectId>>,
    pub resource_ids: Option<Vec<ResourceId>>,
    pub integration_ids: Option<Vec<IntegrationId>>,
    pub movable_object_ids: Option<Vec<PlanningObjectId>>,
    pub max_mutations: u32,
}
fn includes<T: PartialEq>(filter: &Option<Vec<T>>, value: &T) -> bool {
    filter.as_ref().is_none_or(|values| values.contains(value))
}
impl PlanningScope {
    /// Scope only filters; it never grants capability or authorization.
    pub fn selects(&self, object: &PlanningObject) -> bool {
        includes(&self.calendar_ids, &object.calendar_id)
            && includes(&self.object_ids, &object.id)
            && includes(&self.integration_ids, &object.integration_id)
            && self.resource_ids.as_ref().is_none_or(|resources| {
                !resources.is_empty()
                    && !object.resource_ids.is_empty()
                    && object.resource_ids.iter().all(|id| resources.contains(id))
            })
    }
    pub fn allows_placement(&self, object: &PlanningObject, range: TimeRange) -> bool {
        self.selects(object)
            && self.time_range.contains_range(range)
            && (object.revision.is_none() || includes(&self.movable_object_ids, &object.id))
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContextSnapshot {
    pub revision: Revision,
    pub captured_at: Instant,
    pub objects: Vec<PlanningObject>,
    pub facts: Vec<Fact>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SearchBudget {
    pub max_candidates: u32,
    pub max_repairs: u32,
    pub max_moved_objects: u32,
    pub max_depth: u32,
}
impl Default for SearchBudget {
    fn default() -> Self {
        Self {
            max_candidates: 256,
            max_repairs: 0,
            max_moved_objects: 1,
            max_depth: 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlanningRequest {
    pub schema_version: SchemaVersion,
    pub request_id: RequestId,
    pub trace_id: TraceId,
    pub principal_id: PrincipalId,
    pub operation: Operation,
    pub scope: PlanningScope,
    pub context: ContextSnapshot,
    pub target_ids: Vec<PlanningObjectId>,
    pub duration: EvidenceField<Duration>,
    pub constraints: Vec<ConstraintSpec>,
    pub preferences: PreferenceProfile,
    pub policy: PolicyContext,
    pub planning_capability: PlanningCapability,
    pub granularity: Duration,
    pub budget: SearchBudget,
}
impl PlanningRequest {
    pub fn object(&self, id: &PlanningObjectId) -> Option<&PlanningObject> {
        self.context.objects.iter().find(|o| &o.id == id)
    }
    pub fn analysis_only(&self) -> bool {
        self.scope.max_mutations == 0
            || matches!(self.operation, Operation::Analyze | Operation::FindSlot)
    }
}
