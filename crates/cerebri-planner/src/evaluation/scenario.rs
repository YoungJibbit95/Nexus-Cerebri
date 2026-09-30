//! Pure source projection. Only this constructor can create a BSF payload; raw JSON
//! is never an input. The private JSON tree is an output assembly detail, not a wire parser.
use super::*;
use crate::*;
use cerebri_constraints::HardConstraint;
use cerebri_preferences::PreferenceSource;
use cerebri_temporal::{Duration, TimeRange};
use cerebri_types::{FieldState, Knowledge, Provenance};
use serde::Serialize;
use serde_json::{Value, json};

pub(crate) type ProjectionResult<T> = std::result::Result<T, EvaluationContractError>;
pub(crate) fn unsigned(v: u64) -> CanonicalU64Decimal {
    CanonicalU64Decimal::new(v).expect("u64 source")
}
pub(crate) fn optional<T>(v: Option<T>) -> CanonicalOptionalV1<T> {
    match v {
        Some(value) => CanonicalOptionalV1::Some { value },
        None => CanonicalOptionalV1::None {},
    }
}
pub(crate) fn semantic_set<T: Serialize>(values: Vec<T>) -> ProjectionResult<Vec<T>> {
    let mut keyed = values
        .into_iter()
        .map(|v| Ok((canonical_json_bytes(&v)?, v)))
        .collect::<ProjectionResult<Vec<_>>>()?;
    keyed.sort_by(|a, b| a.0.cmp(&b.0));
    keyed.dedup_by(|a, b| a.0 == b.0);
    Ok(keyed.into_iter().map(|(_, v)| v).collect())
}
pub(crate) fn provenance(v: Provenance) -> ProvenanceToken {
    match v {
        Provenance::UserExplicit => ProvenanceToken::UserExplicit,
        Provenance::IntegrationFact => ProvenanceToken::IntegrationFact,
        Provenance::SystemFact => ProvenanceToken::SystemFact,
        Provenance::PersonalLearned => ProvenanceToken::PersonalLearned,
        Provenance::GlobalLearned => ProvenanceToken::GlobalLearned,
        Provenance::ModelInference => ProvenanceToken::ModelInference,
        Provenance::Default => ProvenanceToken::Default,
    }
}
pub(crate) fn preference_source(v: PreferenceSource) -> PreferenceSourceToken {
    match v {
        PreferenceSource::ExplicitCurrentRequest => PreferenceSourceToken::ExplicitCurrentRequest,
        PreferenceSource::SessionContext => PreferenceSourceToken::SessionContext,
        PreferenceSource::PersonalLearned => PreferenceSourceToken::PersonalLearned,
        PreferenceSource::GlobalLearned => PreferenceSourceToken::GlobalLearned,
        PreferenceSource::Default => PreferenceSourceToken::Default,
    }
}
// Frozen architecture precedence, independent of token spelling and enum order.
fn preference_source_rank(source: PreferenceSourceToken) -> u8 {
    match source {
        PreferenceSourceToken::ExplicitCurrentRequest => 0,
        PreferenceSourceToken::SessionContext => 1,
        PreferenceSourceToken::PersonalLearned => 2,
        PreferenceSourceToken::GlobalLearned => 3,
        PreferenceSourceToken::Default => 4,
    }
}
fn mutation(v: MutationKind) -> MutationKindToken {
    match v {
        MutationKind::CreateEvent => MutationKindToken::CreateEvent,
        MutationKind::MoveEvent => MutationKindToken::MoveEvent,
        MutationKind::UpdateEvent => MutationKindToken::UpdateEvent,
        MutationKind::DeleteEvent => MutationKindToken::DeleteEvent,
    }
}
pub(crate) fn instant(v: cerebri_temporal::Instant) -> ProjectionResult<CanonicalInstantV1> {
    v.try_into()
}
fn range(v: TimeRange) -> ProjectionResult<CanonicalTimeRangeV1> {
    v.try_into()
}
pub(crate) fn seconds(v: Duration) -> ProjectionResult<CanonicalPositiveSeconds> {
    CanonicalPositiveSeconds::new(
        u64::try_from(v.as_seconds()).map_err(|_| EvaluationContractError("negative duration"))?,
    )
}
pub(crate) fn knowledge<T>(v: &FieldState<T>) -> (KnowledgeStateToken, Vec<&T>) {
    use KnowledgeStateToken as K;
    match v {
        FieldState::Unresolved => (K::Unresolved, vec![]),
        FieldState::Resolved(Knowledge::Missing) => (K::Missing, vec![]),
        FieldState::Resolved(Knowledge::Unknown) => (K::Unknown, vec![]),
        FieldState::Resolved(Knowledge::Known(v)) => (K::Known, vec![v]),
        FieldState::Resolved(Knowledge::Uncertain {
            value,
            confidence: _,
        }) => (K::Uncertain, vec![value]),
        FieldState::Resolved(Knowledge::Ambiguous(v)) => (K::Ambiguous, v.iter().collect()),
    }
}
pub(crate) fn alias(
    bindings: &[IdentityBindingV1],
    kind: IdentityTypeV1,
    id: &str,
) -> ProjectionResult<CanonicalAlias> {
    bindings
        .iter()
        .find(|b| b.identity_type == kind && b.source_id.as_str() == id)
        .map(|b| b.alias.clone())
        .ok_or(EvaluationContractError(
            "missing canonical identity binding",
        ))
}
pub(crate) fn refs<'a>(
    bindings: &[IdentityBindingV1],
    kind: IdentityTypeV1,
    ids: impl Iterator<Item = &'a str>,
) -> ProjectionResult<Vec<CanonicalAlias>> {
    let mut result = ids
        .map(|id| alias(bindings, kind, id))
        .collect::<ProjectionResult<Vec<_>>>()?;
    result.sort();
    result.dedup();
    Ok(result)
}
pub(crate) fn fact_refs(
    bindings: &[IdentityBindingV1],
    ids: &[cerebri_types::FactId],
) -> ProjectionResult<Vec<CanonicalAlias>> {
    refs(
        bindings,
        IdentityTypeV1::FactId,
        ids.iter().map(|id| id.as_str()),
    )
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "state")]
enum CanonicalOptionalRefSetV1 {
    #[serde(rename = "UNBOUNDED")]
    Unbounded,
    #[serde(rename = "BOUNDED")]
    Bounded { values: Vec<CanonicalAlias> },
}
fn filter<'a>(
    b: &[IdentityBindingV1],
    kind: IdentityTypeV1,
    ids: Option<impl Iterator<Item = &'a str>>,
) -> ProjectionResult<CanonicalOptionalRefSetV1> {
    Ok(match ids {
        None => CanonicalOptionalRefSetV1::Unbounded,
        Some(ids) => CanonicalOptionalRefSetV1::Bounded {
            values: refs(b, kind, ids)?,
        },
    })
}
pub(crate) fn preferences(
    request: &PlanningRequest,
    b: &[IdentityBindingV1],
) -> ProjectionResult<Vec<Value>> {
    let mut entries = Vec::new();
    for p in &request.preferences.preferences {
        let source = preference_source(p.source);
        if matches!(
            source,
            PreferenceSourceToken::PersonalLearned | PreferenceSourceToken::GlobalLearned
        ) {
            return Err(EvaluationContractError(
                "learned preference source forbidden in E2",
            ));
        }
        let at = instant(p.preferred_start)?;
        let evidence = fact_refs(b, &p.evidence)?;
        entries.push((source, at, evidence));
    }
    entries.sort_by(|(a, at, ar), (b, bt, br)| {
        (preference_source_rank(*a), at, ar).cmp(&(preference_source_rank(*b), bt, br))
    });
    entries.dedup();
    Ok(entries.into_iter().map(|(source,at,evidence)|json!({"source":source,"preferred_start":at,"evidence_fact_refs":evidence})).collect())
}
fn rule(
    v: &HardConstraint,
    b: &[IdentityBindingV1],
) -> ProjectionResult<CanonicalHardConstraintRuleV1> {
    use CanonicalHardConstraintRuleV1 as C;
    Ok(match v {
        HardConstraint::NoOverlap => C::NoOverlap {},
        HardConstraint::ExplicitTime(v) => C::ExplicitTime { range: range(*v)? },
        HardConstraint::ExplicitDate(v) => C::ExplicitDate {
            date: (*v).try_into()?,
        },
        HardConstraint::EarliestStart(v) => C::EarliestStart { at: instant(*v)? },
        HardConstraint::LatestEnd(v) => C::LatestEnd { at: instant(*v)? },
        HardConstraint::Deadline(v) => C::Deadline { at: instant(v.0)? },
        HardConstraint::MinDuration(v) => C::MinDuration {
            seconds: seconds(*v)?,
        },
        HardConstraint::FixedDuration(v) => C::FixedDuration {
            seconds: seconds(*v)?,
        },
        HardConstraint::AvailabilityWindow(v) => C::AvailabilityWindow { range: range(*v)? },
        HardConstraint::DependencyOrder(v) => C::DependencyOrder {
            object: alias(b, IdentityTypeV1::PlanningObjectId, v.as_str())?,
        },
        HardConstraint::RequiredBuffer(v) => C::RequiredBuffer {
            seconds: seconds(*v)?,
        },
        HardConstraint::RecurrenceRule => C::RecurrenceRule {},
        HardConstraint::TimezoneIntegrity(v) => C::TimezoneIntegrity {
            timezone: (*v).into(),
        },
        HardConstraint::ExternalLock {
            provenance: p,
            reason: _,
        } => C::ExternalLock {
            provenance: provenance(*p),
        },
    })
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct BaseScenarioPayloadV1(Value);
closed_wire! {
/// Source revisions accompanying the selected labeling, never part of BSF.
pub struct RevisionBindingV1 {
    pub alias: CanonicalAlias,
    pub entity_type: RevisionEntityTypeV1,
    pub revision: CanonicalOptionalV1<CanonicalU64Decimal>,
}
}
#[derive(Debug, Clone)]
pub struct BaseScenarioProjectionV1 {
    payload: BaseScenarioPayloadV1,
    identity_bindings: Vec<IdentityBindingV1>,
    revision_bindings: Vec<RevisionBindingV1>,
}
impl BaseScenarioPayloadV1 {
    pub fn canonical_bytes(&self) -> ProjectionResult<Vec<u8>> {
        canonical_json_bytes(self)
    }
    pub fn fingerprint(&self) -> ProjectionResult<SHA256Hex> {
        Ok(FingerprintDomainV1::BaseScenario.digest_bytes(&self.canonical_bytes()?))
    }
}
impl BaseScenarioProjectionV1 {
    /// Pure evaluation projection; neither planner admission nor authorization.
    /// No projection or fingerprint is returned on a non-fingerprintable source.
    pub fn from_request(request: &PlanningRequest) -> ProjectionResult<Self> {
        pre_fingerprintability(request)?;
        let source = super::source_graph::project(request)?;
        let labeled = source
            .canonicalize_with_payload(|graph, b| project(request, graph, b)?.canonical_bytes())?;
        let payload = project(request, &labeled.graph, &labeled.identity_bindings)?;
        let mut revisions = Vec::new();
        for o in &request.context.objects {
            revisions.push(RevisionBindingV1 {
                alias: alias(
                    &labeled.identity_bindings,
                    IdentityTypeV1::PlanningObjectId,
                    o.id.as_str(),
                )?,
                entity_type: RevisionEntityTypeV1::PlanningObject,
                revision: optional(o.revision.map(|r| unsigned(r.0))),
            });
        }
        if let Some(temporal) = &request.context.temporal {
            for s in &temporal.series {
                revisions.push(RevisionBindingV1 {
                    alias: alias(
                        &labeled.identity_bindings,
                        IdentityTypeV1::SeriesId,
                        s.id.as_str(),
                    )?,
                    entity_type: RevisionEntityTypeV1::TemporalSeries,
                    revision: optional(match s.state {
                        SeriesState::Existing { revision } => Some(unsigned(revision.0)),
                        SeriesState::Prospective => None,
                    }),
                });
            }
        }
        revisions.sort_by(|a, b| a.alias.cmp(&b.alias));
        Ok(Self {
            payload,
            identity_bindings: labeled.identity_bindings,
            revision_bindings: revisions,
        })
    }
    pub fn payload(&self) -> &BaseScenarioPayloadV1 {
        &self.payload
    }
    pub fn identity_bindings(&self) -> &[IdentityBindingV1] {
        &self.identity_bindings
    }
    pub fn revision_bindings(&self) -> &[RevisionBindingV1] {
        &self.revision_bindings
    }
}
fn pre_fingerprintability(request: &PlanningRequest) -> ProjectionResult<()> {
    use cerebri_types::SchemaVersion;
    use std::collections::BTreeSet;
    if !matches!(
        request.schema_version,
        SchemaVersion::CPIR_0_1 | SchemaVersion::CPIR_0_2
    ) {
        return Err(EvaluationContractError("FINGERPRINT_SCHEMA_UNSUPPORTED"));
    }
    let mut targets = BTreeSet::new();
    if request.target_ids.iter().any(|id| !targets.insert(id)) {
        return Err(EvaluationContractError("DUPLICATE_TARGET"));
    }
    // Reject unsupported evidence before entering the bounded labeling search.
    CanonicalDurationKnowledgeV1::try_from(&request.duration.value)?;
    if request.preferences.preferences.iter().any(|p| {
        matches!(
            p.source,
            PreferenceSource::PersonalLearned | PreferenceSource::GlobalLearned
        )
    }) {
        return Err(EvaluationContractError(
            "learned preference source forbidden in E2",
        ));
    }
    // Native bounded compilation is the single occurrence-identity authority.
    // Only IdentityCollision is a pre-BSF failure here. Ordinary representable
    // compilation/admission failures remain fingerprintable under section 50.
    if matches!(
        compile_snapshot(
            &request.context,
            cerebri_temporal::PlanningHorizon(request.scope.time_range),
        ),
        Err(CompilationError::IdentityCollision(_))
    ) {
        return Err(EvaluationContractError("COMPILATION_IDENTITY_COLLISION"));
    }
    Ok(())
}
fn project(
    request: &PlanningRequest,
    graph: &CanonicalGraphSerializationV1,
    b: &[IdentityBindingV1],
) -> ProjectionResult<BaseScenarioPayloadV1> {
    // Exhaustive source registry: adding a request field requires an explicit projection decision.
    let PlanningRequest {
        schema_version,
        request_id: _,
        trace_id: _,
        principal_id: _,
        operation,
        scope,
        context,
        target_ids,
        duration,
        constraints,
        preferences: _,
        policy,
        planning_capability: capability,
        granularity,
        budget: _,
    } = request;
    let operation = match operation {
        Operation::Create => OperationToken::Create,
        Operation::Move => OperationToken::Move,
        Operation::Update => OperationToken::Update,
        Operation::Cancel => OperationToken::Cancel,
        Operation::FindSlot => OperationToken::FindSlot,
        Operation::Reschedule => OperationToken::Reschedule,
        Operation::Optimize => OperationToken::Optimize,
        Operation::Plan => OperationToken::Plan,
        Operation::Analyze => OperationToken::Analyze,
    };
    let targets = refs(
        b,
        IdentityTypeV1::PlanningObjectId,
        target_ids.iter().map(|id| id.as_str()),
    )?;
    let knowledge = CanonicalDurationKnowledgeV1::try_from(&duration.value)?;
    let constraints = constraints.iter().map(|c|Ok(json!({"object":alias(b,IdentityTypeV1::PlanningObjectId,c.object_id.as_str())?,"rule":rule(&c.rule,b)?,"evidence_fact_refs":fact_refs(b,&c.evidence)?}))).collect::<ProjectionResult<Vec<_>>>()?;
    let temporal = match &context.temporal {
        None => json!({"state":"ABSENT"}),
        Some(t) => {
            let mut series = t
                .series
                .iter()
                .map(|s| {
                    Ok(CanonicalTemporalSeriesWireV1 {
                        series: alias(b, IdentityTypeV1::SeriesId, s.id.as_str())?,
                        state: match s.state {
                            SeriesState::Existing { .. } => SeriesStateToken::Existing,
                            SeriesState::Prospective => SeriesStateToken::Prospective,
                        },
                        provenance: provenance(s.provenance),
                        evidence_fact_refs: fact_refs(b, &s.evidence)?,
                        rule: (&s.rule).try_into()?,
                    })
                })
                .collect::<ProjectionResult<Vec<_>>>()?;
            series.sort_by(|a, b| a.series.cmp(&b.series));
            let series = series
                .into_iter()
                .map(CanonicalTemporalSeriesV1::try_from)
                .collect::<ProjectionResult<Vec<_>>>()?;
            let coverage = match t.coverage {
                cerebri_temporal::Coverage::Complete => CoverageToken::Complete,
                cerebri_temporal::Coverage::Incomplete => CoverageToken::Incomplete,
            };
            json!({"state":"PRESENT","horizon":range(t.horizon.0)?,"coverage":coverage,"series":series})
        }
    };
    let mode = match policy.snapshot.mode {
        DeploymentMode::Local => DeploymentModeToken::Local,
        DeploymentMode::Test => DeploymentModeToken::Test,
        DeploymentMode::Shadow => DeploymentModeToken::Shadow,
        DeploymentMode::Suggestion => DeploymentModeToken::Suggestion,
        DeploymentMode::Confirmation => DeploymentModeToken::Confirmation,
        DeploymentMode::LimitedAutomation => DeploymentModeToken::LimitedAutomation,
    };
    let grants = capability.mutations.iter().map(|g|Ok(json!({"object":alias(b,IdentityTypeV1::PlanningObjectId,g.object_id.as_str())?,
        "calendar":alias(b,IdentityTypeV1::CalendarId,g.calendar_id.as_str())?,"integration":alias(b,IdentityTypeV1::IntegrationId,g.integration_id.as_str())?,"kind":mutation(g.kind)}))).collect::<ProjectionResult<Vec<_>>>()?;
    Ok(BaseScenarioPayloadV1(json!({
        "schema_version":"1", "cpir_semantic_version":{"major":unsigned(u64::from(schema_version.major)),"minor":unsigned(u64::from(schema_version.minor))},
        "operation":operation,"granularity_seconds":seconds(*granularity)?, "graph":graph,
        "request_duration":{"knowledge":knowledge,"provenance":provenance(duration.provenance),"evidence_fact_refs":fact_refs(b,&duration.evidence)?},
        "targets":targets.into_iter().map(|object|json!({"object":object,"role":"TARGET"})).collect::<Vec<_>>(),
        "scope":{"time_range":range(scope.time_range)?,"max_mutations":unsigned(u64::from(scope.max_mutations)),
            "calendar_ids":filter(b,IdentityTypeV1::CalendarId,scope.calendar_ids.as_ref().map(|v|v.iter().map(|id|id.as_str())))?,
            "object_ids":filter(b,IdentityTypeV1::PlanningObjectId,scope.object_ids.as_ref().map(|v|v.iter().map(|id|id.as_str())))?,
            "integration_ids":filter(b,IdentityTypeV1::IntegrationId,scope.integration_ids.as_ref().map(|v|v.iter().map(|id|id.as_str())))?,
            "resource_ids":filter(b,IdentityTypeV1::ResourceId,scope.resource_ids.as_ref().map(|v|v.iter().map(|id|id.as_str())))?,
            "movable_object_ids":filter(b,IdentityTypeV1::PlanningObjectId,scope.movable_object_ids.as_ref().map(|v|v.iter().map(|id|id.as_str())))?},
        "hard_constraints":semantic_set(constraints)?,"preferences":preferences(request,b)?,"temporal_context":temporal,
        "planning_policy":{"allow_uncertain_duration":policy.snapshot.allow_uncertain_duration,"confirmation":{"all_mutations":policy.snapshot.confirmation.all_mutations,"deletion":policy.snapshot.confirmation.deletion},
            "mode":mode,"mutation":{"allowed_actions":semantic_set(policy.snapshot.mutation.allowed_actions.iter().map(|v|mutation(*v)).collect())?,"max_mutations":unsigned(u64::from(policy.snapshot.mutation.max_mutations))}},
        "planning_capability":{"read":capability.read,"plan":capability.plan,"mutations":semantic_set(grants)?}
    })))
}
