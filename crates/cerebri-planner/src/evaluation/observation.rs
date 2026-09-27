//! Closed nested Episode wire shapes. Production observation, policy application,
//! search-set/proof validation and lifecycle reduction belong to later phases.
use super::*;
use cerebri_preferences::RankingFeatureSet;
use cerebri_temporal::Instant;
use serde::{Deserialize, Serialize};

/// Episode-local candidate identity; never used to define candidate semantics.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CandidateId(IdentifierString);
impl CandidateId {
    pub fn new(value: impl Into<String>) -> Result<Self, EvaluationContractError> {
        IdentifierString::new(value).map(Self)
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchHypothesisSchemaVersion {
    #[serde(rename = "0.1")]
    V0_1,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchVisitSchemaVersion {
    #[serde(rename = "0.1")]
    V0_1,
}

closed_wire! {
/// Placement wire shape (§51.5); no digest writer or planning validation.
pub struct DigestPlacementV1 {
    pub object_id: IdentifierString,
    #[serde(deserialize_with = "object_only")]
    pub start: CanonicalInstantV1,
    #[serde(deserialize_with = "object_only")]
    pub end: CanonicalInstantV1,
}
}

closed_wire! {
pub struct CanonicalSearchHypothesisV0_1 {
    pub schema_version: SearchHypothesisSchemaVersion,
    pub placements: [DigestPlacementV1; 1],
}
}

closed_wire! {
pub struct SearchVisitObservationV0_1 {
    pub schema_version: SearchVisitSchemaVersion,
    pub hypothesis_schema_version: SearchHypothesisSchemaVersion,
    pub declared_hypotheses: Vec<CanonicalSearchHypothesisV0_1>,
    pub visited_hypotheses: Vec<CanonicalSearchHypothesisV0_1>,
    pub declared_hypothesis_count: CanonicalU64,
    pub evaluated_positions: CanonicalU64,
    pub declared_set_identity_digest: SHA256Hex,
    pub visited_set_identity_digest: SHA256Hex,
    pub visited_order_digest: SHA256Hex,
}
}

closed_wire! {
pub struct PlannerAdmissionObservationV0_1 {
    pub admission_class: PlannerAdmissionClassToken,
    pub issue_codes: Vec<PlannerAdmissionIssueCodeToken>,
}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotAdmittedOutcomeV0_1 {
    #[serde(rename = "INSUFFICIENT_INFORMATION")]
    InsufficientInformation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "state",
    rename_all = "SCREAMING_SNAKE_CASE",
    deny_unknown_fields
)]
pub enum CandidateGenerationObservationV0_1 {
    NotAdmitted {
        planner_admission: PlannerAdmissionObservationV0_1,
        outcome: NotAdmittedOutcomeV0_1,
    },
    Searched {
        planner_admission: PlannerAdmissionObservationV0_1,
        search_visit: SearchVisitObservationV0_1,
        outcome: PlanningOutcomeToken,
        search_assessment: SearchAssessmentToken,
        search_exhausted: bool,
        proof_claim: ProofClaimToken,
    },
}

closed_wire! {
/// Native numeric fields intentionally retain the qualified Slice-1 representation.
/// `shift_seconds` is the Episode field name, not a change to the planner transport.
pub struct DeterministicOrderingKeyV0_1 {
    pub preference_distance_seconds: u64,
    pub mutation_count: u32,
    pub shift_seconds: u64,
    pub start: Instant,
    pub object_id: IdentifierString,
}
}

closed_wire! {
pub struct CandidateObservationV0_1 {
    pub candidate_id: CandidateId,
    pub candidate_fingerprint: SHA256Hex,
    pub placements: [DigestPlacementV1; 1],
    #[serde(deserialize_with = "object_only")]
    pub ranking_feature_snapshot: RankingFeatureSet,
    pub deterministic_rank_in_discovered: CanonicalU64,
    pub deterministic_ordering_key: DeterministicOrderingKeyV0_1,
}
}

closed_wire! {
pub struct ProductEligibilityRecordV0_1 {
    pub candidate_id: CandidateId,
    pub eligible: bool,
    #[serde(deserialize_with = "required_nullable")]
    pub exclusion_rule_id: Option<RuleIdV1>,
}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "state",
    rename_all = "SCREAMING_SNAKE_CASE",
    deny_unknown_fields
)]
pub enum ProductEligibilityObservationV0_1 {
    NotApplicable {
        reason: ProductDecisionReasonToken,
    },
    Applied {
        records: Vec<ProductEligibilityRecordV0_1>,
    },
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "state",
    rename_all = "SCREAMING_SNAKE_CASE",
    deny_unknown_fields
)]
pub enum RankingObservationV0_1 {
    NotApplicable {
        reason: ProductDecisionReasonToken,
    },
    Applied {
        ranked_eligible_order: Vec<CandidateId>,
    },
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "state",
    rename_all = "SCREAMING_SNAKE_CASE",
    deny_unknown_fields
)]
pub enum DisplayDecisionV0_1 {
    NotApplicable {
        reason: ProductDecisionReasonToken,
    },
    Applied {
        display_policy_output_order: Vec<CandidateId>,
    },
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "state",
    rename_all = "SCREAMING_SNAKE_CASE",
    deny_unknown_fields
)]
pub enum ExposureObservationV0_1 {
    NotApplicable {
        reason: ProductDecisionReasonToken,
    },
    NotExposed {
        reason: IdentifierString,
    },
    Exposed {
        surface_id: IdentifierString,
        surface_version: IdentifierString,
        displayed_order: Vec<CandidateId>,
        #[serde(deserialize_with = "required_nullable")]
        viewport_visible_candidate_ids: Option<Vec<CandidateId>>,
        exposed_at: Instant,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdmittedValidationClassV0_1 {
    #[serde(rename = "VALID")]
    Valid,
    #[serde(rename = "VALID_WITH_UNCERTAINTY")]
    ValidWithUncertainty,
}
closed_wire! {
pub struct ManualReplacementObservationV0_1 {
    pub replacement_id: IdentifierString,
    pub observed_at: Instant,
    pub provenance: ProvenanceToken,
    pub placements: Vec<DigestPlacementV1>,
    pub validation_admission_class: AdmittedValidationClassV0_1,
    #[serde(deserialize_with = "object_only")]
    pub ranking_feature_snapshot: RankingFeatureSet,
}
}

closed_wire! {
pub struct InteractionEventV0_1 {
    pub sequence: CanonicalU64,
    pub at: Instant,
    pub kind: InteractionKindToken,
    #[serde(deserialize_with = "required_nullable")]
    pub candidate_id: Option<CandidateId>,
    #[serde(deserialize_with = "required_nullable")]
    pub replacement_id: Option<IdentifierString>,
    #[serde(deserialize_with = "required_nullable")]
    pub rejection_scope: Option<RejectionScopeToken>,
    #[serde(deserialize_with = "required_nullable")]
    pub correction_kind: Option<CorrectionKindToken>,
}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "state",
    rename_all = "SCREAMING_SNAKE_CASE",
    deny_unknown_fields
)]
pub enum CaptureCompletenessV0_1 {
    Complete {},
    Incomplete {
        reasons: Vec<CaptureIncompleteReason>,
    },
}
