//! Deterministic input identity (§23), not admission, authorization or replay evidence.
use super::*;
use crate::{PlanningRequest, SearchBudget};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecisionInputSchemaVersionV1 {
    #[serde(rename = "1")]
    V1,
}

/// Reserved by the generic grammar; E2 has no accepted experiment value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExperimentAssignmentIdentityV1 {}

closed_wire! {
pub struct CanonicalSearchBudgetV1 {
    pub max_candidates: CanonicalU64Decimal,
    pub max_depth: CanonicalU64Decimal,
    pub max_moved_objects: CanonicalU64Decimal,
    pub max_repairs: CanonicalU64Decimal,
}
}
impl From<SearchBudget> for CanonicalSearchBudgetV1 {
    fn from(budget: SearchBudget) -> Self {
        let SearchBudget {
            max_candidates,
            max_depth,
            max_moved_objects,
            max_repairs,
        } = budget;
        Self {
            max_candidates: scenario::unsigned(u64::from(max_candidates)),
            max_depth: scenario::unsigned(u64::from(max_depth)),
            max_moved_objects: scenario::unsigned(u64::from(max_moved_objects)),
            max_repairs: scenario::unsigned(u64::from(max_repairs)),
        }
    }
}
closed_wire! {
pub struct CanonicalExpansionLimitsV1 {
    pub max_dates: CanonicalU64Decimal,
    pub max_occurrences: CanonicalU64Decimal,
}
}
closed_wire! {
pub struct CanonicalPolicyIdentityV1 {
    pub policy_set_id: IdentifierString,
    pub policy_version: CanonicalU64Decimal,
}
}

/// Semantic identities supplied by a trusted artifact-identity boundary.
/// The caller must verify identities before use; this phase neither authenticates
/// artifacts nor accepts their filesystem/network/replay resolver bindings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionInputArtifactsV1 {
    pub planner_artifact: ArtifactSemanticIdentityV0_1,
    pub generator_artifact: ArtifactSemanticIdentityV0_1,
    pub generator_configuration_artifact: CanonicalOptionalV1<ArtifactSemanticIdentityV0_1>,
    pub product_eligibility_policy: ArtifactSemanticIdentityV0_1,
    pub ranking_policy: ArtifactSemanticIdentityV0_1,
    pub display_policy: ArtifactSemanticIdentityV0_1,
}

closed_wire! {
/// Editable wire/conformance input. Convert fallibly to `DecisionInputPayloadV1`.
/// Production callers should use `DecisionInputProjectionV1::from_request` so
/// the BSF, bindings and work observation all come from the same fresh source.
pub struct DecisionInputWireV1 {
    pub schema_version: DecisionInputSchemaVersionV1,
    pub base_scenario_fingerprint: SHA256Hex,
    pub identity_bindings: Vec<IdentityBindingV1>,
    pub revision_bindings: Vec<RevisionBindingV1>,
    pub context_source_revision: CanonicalU64Decimal,
    pub planner_artifact: ArtifactSemanticIdentityV0_1,
    pub generator_artifact: ArtifactSemanticIdentityV0_1,
    pub generator_configuration_artifact: CanonicalOptionalV1<ArtifactSemanticIdentityV0_1>,
    pub search_budget: CanonicalSearchBudgetV1,
    pub temporal_expansion_limits: CanonicalOptionalV1<CanonicalExpansionLimitsV1>,
    pub policy_identity: CanonicalPolicyIdentityV1,
    pub active_preferences: Vec<CanonicalPreferenceEvidenceV1>,
    pub product_eligibility_policy: ArtifactSemanticIdentityV0_1,
    pub ranking_policy: ArtifactSemanticIdentityV0_1,
    pub display_policy: ArtifactSemanticIdentityV0_1,
    pub model_artifact: CanonicalOptionalV1<ArtifactSemanticIdentityV0_1>,
    pub experiment_assignment: CanonicalOptionalV1<ExperimentAssignmentIdentityV1>,
    pub planner_admission_work: PlannerAdmissionWorkObservationV0_1,
}
}

/// Canonical, internally validated payload. Import validation does not authenticate
/// its BSF, source bindings or work measurement; source remeasurement is required.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct DecisionInputPayloadV1(DecisionInputWireV1);

impl<'de> Deserialize<'de> for DecisionInputPayloadV1 {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // DI has only canonical strings, arrays and objects. In particular, reject
        // Serde's alternative externally-tagged unit-enum form {"TOKEN":null}.
        // Preserve duplicate-key rejection before a JSON tree can erase duplicates.
        // This import-only syntax guard does not normalize semantic collections or
        // change the reused Phase-A component deserializers.
        let input =
            super::canonical_input::CanonicalDigestJson::<false>::deserialize(deserializer)?;
        let wire: DecisionInputWireV1 =
            serde_json::from_value(input.0).map_err(serde::de::Error::custom)?;
        wire.try_into().map_err(serde::de::Error::custom)
    }
}

impl TryFrom<DecisionInputWireV1> for DecisionInputPayloadV1 {
    type Error = EvaluationContractError;
    fn try_from(v: DecisionInputWireV1) -> Result<Self, Self::Error> {
        if !matches!(v.model_artifact, CanonicalOptionalV1::None {})
            || !matches!(v.experiment_assignment, CanonicalOptionalV1::None {})
        {
            return Err(EvaluationContractError(
                "E2 model and experiment must be NONE",
            ));
        }
        // The metric's candidate multiplier is the native SearchBudget u32.
        let max_candidates = u32::try_from(v.search_budget.max_candidates.value())
            .map_err(|_| EvaluationContractError("candidate budget outside work metric domain"))?;
        v.planner_admission_work.validate_budget(max_candidates)?;

        let mut identities = BTreeSet::new();
        let mut aliases = BTreeMap::new();
        for (index, binding) in v.identity_bindings.iter().enumerate() {
            let index = u32::try_from(index)
                .map_err(|_| EvaluationContractError("canonical alias out of range"))?;
            if binding.alias != CanonicalAlias::new(index)?
                || !identities.insert((binding.identity_type.as_str(), binding.source_id.as_str()))
            {
                return Err(EvaluationContractError("noncanonical identity bindings"));
            }
            aliases.insert(&binding.alias, binding.identity_type);
        }
        if v.revision_bindings
            .windows(2)
            .any(|p| p[0].alias >= p[1].alias)
        {
            return Err(EvaluationContractError(
                "noncanonical revision binding order",
            ));
        }
        for binding in &v.revision_bindings {
            let expected = match binding.entity_type {
                RevisionEntityTypeV1::PlanningObject => IdentityTypeV1::PlanningObjectId,
                RevisionEntityTypeV1::TemporalSeries => IdentityTypeV1::SeriesId,
            };
            if aliases.get(&binding.alias) != Some(&expected) {
                return Err(EvaluationContractError(
                    "revision binding identity mismatch",
                ));
            }
        }
        for p in &v.active_preferences {
            if matches!(
                p.source,
                PreferenceSourceToken::PersonalLearned | PreferenceSourceToken::GlobalLearned
            ) {
                return Err(EvaluationContractError(
                    "learned preference source forbidden in E2",
                ));
            }
            if p.evidence_fact_refs.windows(2).any(|p| p[0] >= p[1])
                || p.evidence_fact_refs
                    .iter()
                    .any(|a| aliases.get(a) != Some(&IdentityTypeV1::FactId))
            {
                return Err(EvaluationContractError(
                    "noncanonical preference evidence refs",
                ));
            }
        }
        if v.active_preferences
            .windows(2)
            .any(|p| !p[0].canonical_cmp(&p[1]).is_lt())
        {
            return Err(EvaluationContractError("noncanonical preference order"));
        }
        Ok(Self(v))
    }
}
impl DecisionInputPayloadV1 {
    pub fn wire(&self) -> &DecisionInputWireV1 {
        &self.0
    }
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, EvaluationContractError> {
        canonical_json_bytes(self)
    }
    pub fn fingerprint(&self) -> Result<SHA256Hex, EvaluationContractError> {
        Ok(FingerprintDomainV1::DecisionInput.digest_bytes(&self.canonical_bytes()?))
    }
}

/// Coherent production projection from one typed request, with fresh BSF and work.
#[derive(Debug, Clone)]
pub struct DecisionInputProjectionV1 {
    payload: DecisionInputPayloadV1,
}
impl DecisionInputProjectionV1 {
    pub fn from_request(
        request: &PlanningRequest,
        artifacts: DecisionInputArtifactsV1,
    ) -> Result<Self, EvaluationContractError> {
        let base = BaseScenarioProjectionV1::from_request(request)?;
        let active_preferences = scenario::preferences(request, base.identity_bindings())?;
        let temporal_expansion_limits =
            scenario::optional(request.context.temporal.as_ref().map(|t| {
                CanonicalExpansionLimitsV1 {
                    max_dates: scenario::unsigned(u64::from(t.limits.max_dates)),
                    max_occurrences: scenario::unsigned(t.limits.max_occurrences as u64),
                }
            }));
        let payload = DecisionInputWireV1 {
            schema_version: DecisionInputSchemaVersionV1::V1,
            base_scenario_fingerprint: base.payload().fingerprint()?,
            identity_bindings: base.identity_bindings().to_vec(),
            revision_bindings: base.revision_bindings().to_vec(),
            context_source_revision: scenario::unsigned(request.context.revision.0),
            planner_artifact: artifacts.planner_artifact,
            generator_artifact: artifacts.generator_artifact,
            generator_configuration_artifact: artifacts.generator_configuration_artifact,
            search_budget: request.budget.into(),
            temporal_expansion_limits,
            policy_identity: CanonicalPolicyIdentityV1 {
                policy_set_id: IdentifierString::new(request.policy.policy_set_id.as_str())?,
                policy_version: scenario::unsigned(request.policy.policy_version.0),
            },
            active_preferences,
            product_eligibility_policy: artifacts.product_eligibility_policy,
            ranking_policy: artifacts.ranking_policy,
            display_policy: artifacts.display_policy,
            model_artifact: CanonicalOptionalV1::None {},
            experiment_assignment: CanonicalOptionalV1::None {},
            planner_admission_work: PlannerAdmissionWorkObservationV0_1::measure(request),
        }
        .try_into()?;
        Ok(Self { payload })
    }
    pub fn payload(&self) -> &DecisionInputPayloadV1 {
        &self.payload
    }
}
