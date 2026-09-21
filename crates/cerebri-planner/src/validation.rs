use crate::{model::*, policy::MutationKind};
use cerebri_constraints::FactValue;
use cerebri_types::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationState {
    Valid,
    ValidWithUncertainty,
    InsufficientInformation,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationIssue {
    UnsupportedSchema,
    UnsupportedOperation(Operation),
    UnsupportedObject(PlanningObjectId),
    RequiredDuration(RequiredFieldError),
    RequiredTime {
        object_id: PlanningObjectId,
        reason: RequiredFieldError,
    },
    DuplicateObject(PlanningObjectId),
    DuplicateFact(FactId),
    DuplicateTarget(PlanningObjectId),
    UnknownObject(PlanningObjectId),
    ContradictoryFact(FactId),
    InvalidFactSource(FactId),
    ScopeViolation(PlanningObjectId),
    PlanningPermissionDenied,
    MissingPlanningCapability(PlanningObjectId),
    PolicyDenied(PlanningObjectId),
    InvalidTargetState(PlanningObjectId),
    NoTargets,
    UnsupportedSearchBudget,
    UnsupportedTargetCount,
    InputLimit,
    Compilation(crate::CompilationError),
    IncompleteCoverage,
    Dependency(crate::DependencyIssue),
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationReport {
    pub state: ValidationState,
    pub issues: Vec<ValidationIssue>,
}

pub(crate) fn mutation_kind(object: &PlanningObject) -> MutationKind {
    if object.revision.is_some() {
        MutationKind::MoveEvent
    } else {
        MutationKind::CreateEvent
    }
}

pub fn validate_request(request: &PlanningRequest) -> ValidationReport {
    validate_and_compile(request).0
}

// Reuse the immutable compiled view within one planning call. Lifecycle validation
// still validates/recompiles its supplied current snapshot independently.
pub(crate) fn validate_and_compile(
    request: &PlanningRequest,
) -> (ValidationReport, Option<crate::CompiledContextSnapshot>) {
    if !metadata_work_admitted(request) {
        return (
            ValidationReport {
                state: ValidationState::InsufficientInformation,
                issues: vec![ValidationIssue::InputLimit],
            },
            None,
        );
    }
    let mut issues = Vec::new();
    let mut uncertain = false;
    if !matches!(
        request.schema_version,
        SchemaVersion::CPIR_0_1 | SchemaVersion::CPIR_0_2
    ) || (request.schema_version == SchemaVersion::CPIR_0_1
        && request.context.temporal.is_some())
    {
        issues.push(ValidationIssue::UnsupportedSchema);
    }
    if !matches!(
        request.operation,
        Operation::Create
            | Operation::Move
            | Operation::FindSlot
            | Operation::Plan
            | Operation::Analyze
    ) {
        issues.push(ValidationIssue::UnsupportedOperation(request.operation));
    }
    if !request.planning_capability.read || !request.planning_capability.plan {
        issues.push(ValidationIssue::PlanningPermissionDenied);
    }
    if request.context.objects.len() > 256
        || request.context.facts.len() > 1024
        || request.constraints.len() > 1024
        || request.budget.max_candidates > 4096
        || (request.budget.max_candidates as u64)
            * (request.context.objects.len() as u64 + 1)
            * (request.constraints.len() as u64
                + request.context.facts.len() as u64
                + request.context.objects.len() as u64
                + 1)
            > 1_000_000
    {
        return (
            ValidationReport {
                state: ValidationState::InsufficientInformation,
                issues: vec![ValidationIssue::InputLimit],
            },
            None,
        );
    }
    if request.budget.max_depth != 0 || request.budget.max_repairs != 0 {
        issues.push(ValidationIssue::UnsupportedSearchBudget);
    }
    match request
        .duration
        .value
        .required(request.policy.snapshot.allow_uncertain_duration && request.analysis_only())
    {
        Ok((_, is_uncertain)) => uncertain |= is_uncertain,
        Err(reason) => issues.push(ValidationIssue::RequiredDuration(reason)),
    }
    let mut ids = BTreeSet::new();
    for object in &request.context.objects {
        if !ids.insert(object.id.clone()) {
            issues.push(ValidationIssue::DuplicateObject(object.id.clone()));
        }
        if object.revision.is_some() && !object.time.provenance.is_fact_source() {
            issues.push(ValidationIssue::InvalidTargetState(object.id.clone()));
        }
        let required = matches!(
            object.kind,
            PlanningObjectKind::Event
                | PlanningObjectKind::Task(_)
                | PlanningObjectKind::Availability
        ) && (object.revision.is_some() || !request.target_ids.contains(&object.id));
        if required && let Err(reason) = object.time.value.required(false) {
            issues.push(ValidationIssue::RequiredTime {
                object_id: object.id.clone(),
                reason,
            });
        }
    }
    if request.target_ids.is_empty() {
        issues.push(ValidationIssue::NoTargets);
    }
    let mut targets = BTreeSet::new();
    for id in &request.target_ids {
        if !targets.insert(id) {
            issues.push(ValidationIssue::DuplicateTarget(id.clone()));
        }
        match request.object(id) {
            None => issues.push(ValidationIssue::UnknownObject(id.clone())),
            Some(object) => {
                if !matches!(object.kind, PlanningObjectKind::Event) {
                    issues.push(ValidationIssue::UnsupportedObject(id.clone()));
                }
                if !request.scope.selects(object) {
                    issues.push(ValidationIssue::ScopeViolation(id.clone()));
                }
                if (request.operation == Operation::Create && object.revision.is_some())
                    || (request.operation == Operation::Move && object.revision.is_none())
                {
                    issues.push(ValidationIssue::InvalidTargetState(id.clone()));
                }
                if object.revision.is_none()
                    && !matches!(object.time.value, FieldState::Resolved(Knowledge::Missing))
                {
                    // Prospective event timing belongs in constraints, not contradictory snapshot facts.
                    issues.push(ValidationIssue::InvalidTargetState(id.clone()));
                }
                if !request.analysis_only() {
                    let kind = mutation_kind(object);
                    if !request.planning_capability.permits(object, kind) {
                        issues.push(ValidationIssue::MissingPlanningCapability(id.clone()));
                    }
                    if !request.policy.permits(kind) {
                        issues.push(ValidationIssue::PolicyDenied(id.clone()));
                    }
                }
            }
        }
    }
    let mut fact_ids = BTreeSet::new();
    for fact in &request.context.facts {
        if !fact_ids.insert(fact.id.clone()) {
            issues.push(ValidationIssue::DuplicateFact(fact.id.clone()));
        }
        if !fact.provenance.is_fact_source() {
            issues.push(ValidationIssue::InvalidFactSource(fact.id.clone()));
        }
        match request.object(&fact.object_id) {
            None => issues.push(ValidationIssue::UnknownObject(fact.object_id.clone())),
            Some(object) => match fact.value {
                FactValue::ScheduledTime(range) | FactValue::Availability(range)
                    if object.time.value.required(false).map(|(r, _)| *r) != Ok(range) =>
                {
                    issues.push(ValidationIssue::ContradictoryFact(fact.id.clone()));
                }
                _ => {}
            },
        }
    }
    for constraint in &request.constraints {
        if request.object(&constraint.object_id).is_none() {
            issues.push(ValidationIssue::UnknownObject(constraint.object_id.clone()));
        }
    }
    let compilation = match crate::compile_snapshot(
        &request.context,
        cerebri_temporal::PlanningHorizon(request.scope.time_range),
    ) {
        Err(error) => {
            issues.push(ValidationIssue::Compilation(error));
            None
        }
        Ok(Some(compiled)) => {
            if compiled.availability.coverage == cerebri_temporal::Coverage::Incomplete {
                issues.push(ValidationIssue::IncompleteCoverage);
            }
            let count =
                request.context.objects.len() as u64 + compiled.occurrences.len() as u64 + 1;
            if u64::from(request.budget.max_candidates)
                * count
                * (request.constraints.len() as u64 + request.context.facts.len() as u64 + count)
                > 1_000_000
            {
                issues.push(ValidationIssue::InputLimit);
            }
            Some(compiled)
        }
        Ok(None) => None,
    };
    issues.extend(
        crate::dependency_graph(&request.context.objects, &request.constraints)
            .issues
            .into_iter()
            .map(ValidationIssue::Dependency),
    );
    issues.sort_by_key(|issue| serde_json::to_string(issue).expect("serializable issue"));
    issues.dedup();
    let state = if !issues.is_empty() {
        ValidationState::InsufficientInformation
    } else if uncertain {
        ValidationState::ValidWithUncertainty
    } else {
        ValidationState::Valid
    };
    (ValidationReport { state, issues }, compilation)
}

/// Count serialized input without allocating a second copy. This complements the
/// cardinality bound: evidence, scope/grant lists and strings also cost work when
/// requests are hashed or constraints cloned for every candidate (ADR-0013).
fn metadata_work_admitted(request: &PlanningRequest) -> bool {
    struct Counter(usize);
    impl std::io::Write for Counter {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            if bytes.len() > (256 * 1024) - self.0 {
                return Err(std::io::Error::other("planner input limit"));
            }
            self.0 += bytes.len();
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut counter = Counter(0);
    serde_json::to_writer(&mut counter, request).is_ok()
        && (counter.0 as u64) * u64::from(request.budget.max_candidates) <= 16 * 1024 * 1024
}
