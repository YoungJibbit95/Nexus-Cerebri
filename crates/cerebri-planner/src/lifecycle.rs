use crate::*;
use cerebri_constraints::{
    ConstraintSpec, ConstraintViolation, FactValue, HardConstraint, TimedObject, check,
};
use cerebri_temporal::{TimeDelta, TimeRange};
use cerebri_types::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Placement {
    pub object_id: PlanningObjectId,
    pub range: TimeRange,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExplanationLevel {
    Simple,
    Technical,
    Research,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlanReason {
    FeasibleWithinScope,
    PreferredStart(cerebri_preferences::PreferenceSource),
    EarliestTieBreak,
    AnalysisOnly,
    DurationUncertain,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoreComponent {
    pub reason: PlanReason,
    pub cost: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CandidateRejectionReason {
    Scope(PlanningObjectId),
    HardConstraint(ConstraintViolation),
    InvalidDuration(PlanningObjectId),
    MutationLimit,
    MovedObjectLimit,
    InvalidTargets,
    Capability(PlanningObjectId),
    Policy(PlanningObjectId),
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
pub enum PlanError {
    #[error("CPIR validation failed")]
    InvalidRequest(ValidationReport),
    #[error("context changed; revalidation required")]
    Stale,
    #[error("candidate violates planning invariants")]
    Rejected(Vec<CandidateRejectionReason>),
    #[error("analysis-only proposals cannot produce mutating actions")]
    AnalysisOnly,
    #[error("authorization, capability or confirmation denied")]
    Unauthorized,
    #[error("policy changed; revalidation required")]
    PolicyChanged,
}
/// Untrusted proposal: deliberately constructible, never executable or deserializable.
#[derive(Debug, Clone, Serialize)]
pub struct ProposedPlan {
    id: PlanId,
    source_revision: Revision,
    placements: Vec<Placement>,
    #[serde(skip)]
    request: Arc<PlanningRequest>,
}
impl ProposedPlan {
    pub fn from_placements(request: &PlanningRequest, placements: Vec<Placement>) -> Self {
        Self::from_shared(Arc::new(request.clone()), placements)
    }
    pub(crate) fn from_shared(
        request: Arc<PlanningRequest>,
        mut placements: Vec<Placement>,
    ) -> Self {
        placements.sort_by(|a, b| {
            a.object_id
                .cmp(&b.object_id)
                .then(a.range.start().cmp(&b.range.start()))
        });
        let id = plan_id(&request, &placements);
        Self {
            id,
            source_revision: request.context.revision,
            placements,
            request,
        }
    }
    pub fn id(&self) -> &PlanId {
        &self.id
    }
    pub fn placements(&self) -> &[Placement] {
        &self.placements
    }
    pub fn validate(self, current: &ContextSnapshot) -> Result<ValidatedPlan, PlanError> {
        let report = validate_request(&self.request);
        if report.state == ValidationState::InsufficientInformation {
            return Err(PlanError::InvalidRequest(report));
        }
        if &self.request.context != current {
            return Err(PlanError::Stale);
        }
        let reasons = placement_violations(&self.request, &self.placements);
        if !reasons.is_empty() {
            return Err(PlanError::Rejected(reasons));
        }
        Ok(ValidatedPlan { proposed: self })
    }
}
// Canonicalize set-like CPIR arrays so equivalent input order produces stable identities.
fn canonical(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Array(items) => {
            for item in items.iter_mut() {
                canonical(item);
            }
            items.sort_by_key(|item| item.to_string());
        }
        serde_json::Value::Object(map) => {
            for item in map.values_mut() {
                canonical(item);
            }
        }
        _ => {}
    }
}
fn plan_id(request: &PlanningRequest, placements: &[Placement]) -> PlanId {
    let mut value = serde_json::json!({ "request": request, "placements": placements });
    canonical(&mut value);
    let digest = Sha256::digest(value.to_string().as_bytes());
    PlanId::new(format!("p-{digest:x}")).expect("fixed ASCII SHA-256 identity")
}

/// Only deterministic validation can construct this proof.
#[derive(Debug)]
pub struct ValidatedPlan {
    proposed: ProposedPlan,
}
impl ValidatedPlan {
    pub fn proposed(&self) -> &ProposedPlan {
        &self.proposed
    }
    pub fn into_action_plan(self) -> Result<ActionPlan, PlanError> {
        let proposed = self.proposed;
        if proposed.request.analysis_only() {
            return Err(PlanError::AnalysisOnly);
        }
        let mut actions = Vec::new();
        for (index, placement) in proposed.placements.iter().enumerate() {
            let object = proposed
                .request
                .object(&placement.object_id)
                .expect("validated target");
            let kind = crate::validation::mutation_kind(object);
            let action = match kind {
                MutationKind::CreateEvent => EventAction::CreateEvent {
                    range: placement.range,
                },
                _ => EventAction::MoveEvent {
                    range: placement.range,
                },
            };
            let identifier = format!("{}-{index}", proposed.id.as_str());
            actions.push(Action {
                id: ActionId::new(identifier.clone()).expect("bounded generated ID"),
                source_plan: proposed.id.clone(),
                target: object.id.clone(),
                expected_revision: object.revision,
                idempotency_key: IdempotencyKey::new(identifier).expect("bounded generated key"),
                calendar_id: object.calendar_id.clone(),
                integration_id: object.integration_id.clone(),
                confirmation_required: proposed.request.policy.requires_confirmation(kind),
                action,
            });
        }
        Ok(ActionPlan { proposed, actions })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EventAction {
    CreateEvent { range: TimeRange },
    MoveEvent { range: TimeRange },
    UpdateEvent { range: TimeRange },
    DeleteEvent,
}
impl EventAction {
    pub fn kind(&self) -> MutationKind {
        match self {
            Self::CreateEvent { .. } => MutationKind::CreateEvent,
            Self::MoveEvent { .. } => MutationKind::MoveEvent,
            Self::UpdateEvent { .. } => MutationKind::UpdateEvent,
            Self::DeleteEvent => MutationKind::DeleteEvent,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Action {
    pub id: ActionId,
    pub source_plan: PlanId,
    pub target: PlanningObjectId,
    pub expected_revision: Option<Revision>,
    pub idempotency_key: IdempotencyKey,
    pub calendar_id: CalendarId,
    pub integration_id: IntegrationId,
    pub confirmation_required: bool,
    pub action: EventAction,
}
/// Explicit mutations, still not executable. No public constructor or Deserialize.
#[derive(Debug, Serialize)]
pub struct ActionPlan {
    proposed: ProposedPlan,
    actions: Vec<Action>,
}
impl ActionPlan {
    pub fn id(&self) -> &PlanId {
        self.proposed.id()
    }
    pub fn actions(&self) -> &[Action] {
        &self.actions
    }
    pub fn source_request(&self) -> &PlanningRequest {
        &self.proposed.request
    }
    pub fn authorize(
        self,
        current: &AuthorizationContext,
    ) -> Result<AuthorizedActionPlan, PlanError> {
        self.check_authorization(current)?;
        Ok(AuthorizedActionPlan { plan: self })
    }
    fn check_authorization(&self, current: &AuthorizationContext) -> Result<(), PlanError> {
        let request = &self.proposed.request;
        if current.source_revision != request.context.revision {
            return Err(PlanError::Stale);
        }
        if current.policy != request.policy {
            return Err(PlanError::PolicyChanged);
        }
        if current.principal_id != request.principal_id
            || !current.planning_capability.read
            || !current.planning_capability.plan
        {
            return Err(PlanError::Unauthorized);
        }
        for action in &self.actions {
            let object = request
                .object(&action.target)
                .ok_or(PlanError::Unauthorized)?;
            let kind = action.action.kind();
            if !current.policy.permits(kind)
                || !current.planning_capability.permits(object, kind)
                || !current
                    .execution_grants
                    .iter()
                    .any(|g| g.permits(object, kind))
                || (current.policy.requires_confirmation(kind)
                    && !current.confirmed_plan_ids.contains(self.id()))
            {
                return Err(PlanError::Unauthorized);
            }
        }
        Ok(())
    }
}
/// Context-bound authorization, not a durable bearer token. Never Deserialize.
#[derive(Debug)]
pub struct AuthorizedActionPlan {
    plan: ActionPlan,
}
impl AuthorizedActionPlan {
    pub fn plan(&self) -> &ActionPlan {
        &self.plan
    }
    pub fn recheck(
        &self,
        context: &ContextSnapshot,
        authorization: &AuthorizationContext,
    ) -> Result<(), PlanError> {
        if context != &self.plan.proposed.request.context {
            return Err(PlanError::Stale);
        }
        self.plan.check_authorization(authorization)
    }
}

pub(crate) fn placement_violations(
    request: &PlanningRequest,
    placements: &[Placement],
) -> Vec<CandidateRejectionReason> {
    let Ok(compiled) = crate::compile_snapshot(
        &request.context,
        cerebri_temporal::PlanningHorizon(request.scope.time_range),
    ) else {
        return vec![CandidateRejectionReason::InvalidTargets];
    };
    placement_violations_compiled(request, placements, compiled.as_ref())
}

pub(crate) fn placement_violations_compiled(
    request: &PlanningRequest,
    placements: &[Placement],
    compiled: Option<&crate::CompiledContextSnapshot>,
) -> Vec<CandidateRejectionReason> {
    let mut errors = Vec::new();
    let mut ids: Vec<_> = placements.iter().map(|p| p.object_id.clone()).collect();
    ids.sort();
    let mut targets = request.target_ids.clone();
    targets.sort();
    if ids != targets {
        return vec![CandidateRejectionReason::InvalidTargets];
    }
    if !request.analysis_only()
        && (placements.len() > request.scope.max_mutations as usize
            || placements.len() > request.policy.snapshot.mutation.max_mutations as usize)
    {
        errors.push(CandidateRejectionReason::MutationLimit);
    }
    let moved = placements
        .iter()
        .filter(|p| {
            request
                .object(&p.object_id)
                .is_some_and(|o| o.revision.is_some())
        })
        .count();
    if moved > request.budget.max_moved_objects as usize {
        errors.push(CandidateRejectionReason::MovedObjectLimit);
    }
    let mut final_objects = Vec::new();
    for object in &request.context.objects {
        let range = placements
            .iter()
            .find(|p| p.object_id == object.id)
            .map(|p| p.range)
            .or_else(|| object.time.value.required(false).ok().map(|(r, _)| *r));
        if let Some(range) = range {
            final_objects.push(TimedObject {
                id: object.id.clone(),
                range,
                timezone: object.timezone,
            });
        }
    }
    final_objects.sort_by(|a, b| a.id.cmp(&b.id));
    let mut busy: Vec<_> = final_objects
        .iter()
        .filter(|timed| {
            request.object(&timed.id).is_some_and(|o| {
                matches!(
                    o.kind,
                    PlanningObjectKind::Event | PlanningObjectKind::Task(_)
                )
            })
        })
        .cloned()
        .collect();
    if let Some(compiled) = compiled {
        busy.extend(compiled.occurrences.iter().map(|o| TimedObject {
            id: PlanningObjectId::new(o.id.as_str()).expect("validated occurrence ID"),
            range: o.occurrence.range,
            timezone: cerebri_temporal::TimeZoneId::UTC,
        }));
    }
    for placement in placements {
        let Some(object) = request.object(&placement.object_id) else {
            return vec![CandidateRejectionReason::InvalidTargets];
        };
        if !request.scope.allows_placement(object, placement.range) {
            errors.push(CandidateRejectionReason::Scope(object.id.clone()));
        }
        if object.revision.is_some()
            && let Ok((original, _)) = object.time.value.required(false)
            && original.duration() != placement.range.duration()
        {
            errors.push(CandidateRejectionReason::InvalidDuration(object.id.clone()));
        }
        if let Ok((duration, _)) = request
            .duration
            .value
            .required(request.analysis_only() && request.policy.snapshot.allow_uncertain_duration)
            && placement.range.duration() != TimeDelta::seconds(duration.as_seconds())
        {
            errors.push(CandidateRejectionReason::InvalidDuration(object.id.clone()));
        }
        if !request.analysis_only() {
            let kind = crate::validation::mutation_kind(object);
            if !request.planning_capability.permits(object, kind) {
                errors.push(CandidateRejectionReason::Capability(object.id.clone()));
            }
            if !request.policy.permits(kind) {
                errors.push(CandidateRejectionReason::Policy(object.id.clone()));
            }
        }
        let candidate = TimedObject {
            id: object.id.clone(),
            range: placement.range,
            timezone: object.timezone,
        };
        let no_overlap = ConstraintSpec {
            object_id: object.id.clone(),
            rule: HardConstraint::NoOverlap,
            evidence: vec![],
        };
        if let Some(v) = check(&no_overlap, &candidate, None, &busy) {
            errors.push(CandidateRejectionReason::HardConstraint(v));
        }
    }
    // Validate the resulting world, including constraints on unchanged objects.
    let mut constraints = request.constraints.clone();
    for fact in &request.context.facts {
        let rule = match fact.value {
            FactValue::Deadline(deadline) => Some(HardConstraint::Deadline(deadline)),
            FactValue::ExternalLock => Some(HardConstraint::ExternalLock {
                provenance: fact.provenance,
                reason: "source fact".into(),
            }),
            _ => None,
        };
        if let Some(rule) = rule {
            constraints.push(ConstraintSpec {
                object_id: fact.object_id.clone(),
                rule,
                evidence: vec![fact.id.clone()],
            });
        }
    }
    for constraint in &mut constraints {
        constraint.evidence.sort();
        constraint.evidence.dedup();
    }
    constraints.sort_by_key(|c| serde_json::to_string(c).expect("serializable constraint"));
    for specification in constraints {
        if let Some(candidate) = final_objects
            .iter()
            .find(|o| o.id == specification.object_id)
        {
            let original = request
                .object(&candidate.id)
                .and_then(|o| o.time.value.required(false).ok())
                .map(|(r, _)| *r);
            // Recurrence occupancy cannot be a graph node; dependencies refer to planning objects.
            let others = if matches!(specification.rule, HardConstraint::DependencyOrder(_)) {
                &final_objects
            } else {
                &busy
            };
            if let Some(v) = check(&specification, candidate, original, others) {
                errors.push(CandidateRejectionReason::HardConstraint(v));
            }
            if let (Some(compiled), HardConstraint::RequiredBuffer(buffer)) =
                (compiled, &specification.rule)
            {
                let margin = TimeDelta::seconds(buffer.as_seconds());
                let certified = candidate
                    .range
                    .start()
                    .checked_sub_signed(margin)
                    .zip(candidate.range.end().checked_add_signed(margin))
                    .is_some_and(|(start, end)| {
                        start >= compiled.horizon.0.start() && end <= compiled.horizon.0.end()
                    });
                if !certified {
                    errors.push(CandidateRejectionReason::HardConstraint(
                        ConstraintViolation {
                            constraint: specification.rule.clone(),
                            object_id: candidate.id.clone(),
                            reason: cerebri_constraints::ViolationReason::OutsideBounds,
                            evidence: cerebri_constraints::ConstraintEvidence {
                                facts: specification.evidence.clone(),
                                blocking_objects: vec![],
                                blocking_occurrences: vec![],
                            },
                        },
                    ));
                }
            }
        } else {
            errors.push(CandidateRejectionReason::InvalidTargets);
        }
    }
    // Keep materialized occupancy identity distinct from external/planning object identity.
    if let Some(compiled) = compiled {
        for error in &mut errors {
            if let CandidateRejectionReason::HardConstraint(violation) = error {
                violation.evidence.blocking_objects.retain(|id| {
                    if let Some(occurrence) = compiled
                        .occurrences
                        .iter()
                        .find(|o| o.id.as_str() == id.as_str())
                    {
                        violation
                            .evidence
                            .blocking_occurrences
                            .push(occurrence.id.clone());
                        violation.evidence.facts.extend(occurrence.evidence.clone());
                        false
                    } else {
                        true
                    }
                });
                violation.evidence.blocking_occurrences.sort();
                violation.evidence.facts.sort();
                violation.evidence.facts.dedup();
            }
        }
    }
    errors
}
