//! Pure deterministic output projection through display-policy L (§§51–52).
//! No exposure/lifecycle input, planner execution, capture or replay.
use super::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

type Result<T, E = EvaluationContractError> = std::result::Result<T, E>;
fn require(condition: bool, message: &'static str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(EvaluationContractError(message))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecisionObservationSchemaVersionV1 {
    #[serde(rename = "1")]
    V1,
}
// Identical semantic admission shape; retain one native admission authority.
pub type DigestPlannerAdmissionV1 = PlannerAdmissionObservationV0_1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "state",
    rename_all = "SCREAMING_SNAKE_CASE",
    deny_unknown_fields
)]
pub enum DigestGenerationV1 {
    NotAdmitted {
        outcome: NotAdmittedOutcomeV0_1,
        proof_claim: ProofClaimToken,
    },
    Searched {
        declared_hypotheses: Vec<CanonicalSearchHypothesisV0_1>,
        visited_hypotheses: Vec<CanonicalSearchHypothesisV0_1>,
        declared_hypothesis_count: CanonicalU64Decimal,
        evaluated_positions: CanonicalU64Decimal,
        discovered_candidate_count: CanonicalU64Decimal,
        declared_set_identity_digest: SHA256Hex,
        visited_set_identity_digest: SHA256Hex,
        visited_order_digest: SHA256Hex,
        outcome: PlanningOutcomeToken,
        search_assessment: SearchAssessmentToken,
        search_exhausted: bool,
        proof_claim: ProofClaimToken,
    },
}
closed_wire! {
pub struct DigestRankingFeatureSchemaV0_1 {
    pub major: CanonicalU64Decimal,
    pub minor: CanonicalU64Decimal,
}
}
closed_wire! {
pub struct DigestRankingFeatureSetV0_1 {
    pub schema_version: DigestRankingFeatureSchemaV0_1,
    #[serde(deserialize_with = "object_only")]
    pub preferred_start_distance_seconds: CanonicalOptionalV1<CanonicalU64Decimal>,
    #[serde(deserialize_with = "object_only")]
    pub preferred_start_source: CanonicalOptionalV1<PreferenceSourceToken>,
    pub mutation_count: CanonicalU64Decimal,
    pub shift_seconds: CanonicalU64Decimal,
}
}
closed_wire! {
#[derive(PartialOrd, Ord)]
pub struct DigestDeterministicOrderingKeyV1 {
    pub preference_distance_seconds: CanonicalU64Decimal,
    pub mutation_count: CanonicalU64Decimal,
    pub shift_seconds: CanonicalU64Decimal,
    #[serde(deserialize_with = "object_only")]
    pub start: CanonicalInstantV1,
    pub object_id: IdentifierString,
}
}
closed_wire! {
pub struct DigestCandidateV1 {
    pub candidate_fingerprint: SHA256Hex,
    pub placements: [DigestPlacementV1; 1],
    pub ranking_feature_snapshot: DigestRankingFeatureSetV0_1,
    pub deterministic_rank_in_discovered: CanonicalU64Decimal,
    pub deterministic_ordering_key: DigestDeterministicOrderingKeyV1,
}
}
closed_wire! {
pub struct DigestProductEligibilityRecordV1 {
    pub candidate_fingerprint: SHA256Hex,
    pub eligible: bool,
    #[serde(deserialize_with = "object_only")]
    pub exclusion_rule_id: CanonicalOptionalV1<RuleIdV1>,
}
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "state",
    rename_all = "SCREAMING_SNAKE_CASE",
    deny_unknown_fields
)]
pub enum DigestProductEligibilityV1 {
    NotApplicable {
        reason: ProductDecisionReasonToken,
    },
    Applied {
        records: Vec<DigestProductEligibilityRecordV1>,
    },
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "state",
    rename_all = "SCREAMING_SNAKE_CASE",
    deny_unknown_fields
)]
pub enum DigestRankingObservationV1 {
    NotApplicable {
        reason: ProductDecisionReasonToken,
    },
    Applied {
        ranked_eligible_order: Vec<SHA256Hex>,
    },
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "state",
    rename_all = "SCREAMING_SNAKE_CASE",
    deny_unknown_fields
)]
pub enum DigestDisplayDecisionV1 {
    NotApplicable {
        reason: ProductDecisionReasonToken,
    },
    Applied {
        display_policy_output_order: Vec<SHA256Hex>,
    },
}
closed_wire! {
/// Lower-level serialization/conformance input. Stored fingerprints are prevalidated
/// inputs here; use `from_observations` to check native candidate placement hashes.
pub struct DecisionObservationDigestWireV1 {
    pub schema_version: DecisionObservationSchemaVersionV1,
    pub decision_input_fingerprint: SHA256Hex,
    pub planner_admission: DigestPlannerAdmissionV1,
    #[serde(deserialize_with = "object_only")]
    pub generation: DigestGenerationV1,
    pub candidates: Vec<DigestCandidateV1>,
    #[serde(deserialize_with = "object_only")]
    pub product_eligibility: DigestProductEligibilityV1,
    #[serde(deserialize_with = "object_only")]
    pub ranking_observation: DigestRankingObservationV1,
    #[serde(deserialize_with = "object_only")]
    pub display_decision: DigestDisplayDecisionV1,
}
}
/// Canonical digest payload with local materialization/rank/product validation.
/// Import/conformance validation is not artifact authentication or R3 replay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct DecisionObservationDigestPayloadV1(DecisionObservationDigestWireV1);
impl<'de> Deserialize<'de> for DecisionObservationDigestPayloadV1 {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let input = super::canonical_input::CanonicalDigestJson::<true>::deserialize(d)?;
        let wire: DecisionObservationDigestWireV1 =
            serde_json::from_value(input.0).map_err(serde::de::Error::custom)?;
        wire.try_into().map_err(serde::de::Error::custom)
    }
}
impl TryFrom<DecisionObservationDigestWireV1> for DecisionObservationDigestPayloadV1 {
    type Error = EvaluationContractError;
    fn try_from(v: DecisionObservationDigestWireV1) -> Result<Self> {
        validate(&v)?;
        Ok(Self(v))
    }
}
impl DecisionObservationDigestPayloadV1 {
    pub fn wire(&self) -> &DecisionObservationDigestWireV1 {
        &self.0
    }
    pub fn canonical_bytes(&self) -> Result<Vec<u8>> {
        canonical_json_bytes(self)
    }
    pub fn digest(&self) -> Result<SHA256Hex> {
        Ok(FingerprintDomainV1::DecisionObservation.digest_bytes(&self.canonical_bytes()?))
    }
}

fn validate(v: &DecisionObservationDigestWireV1) -> Result<()> {
    require(
        v.planner_admission
            .issue_codes
            .windows(2)
            .all(|p| p[0].as_str() < p[1].as_str()),
        "noncanonical admission issue codes",
    )?;
    require(
        !v.planner_admission.issue_codes.iter().any(|c| {
            matches!(
                c.as_str(),
                "DUPLICATE_OBJECT"
                    | "DUPLICATE_FACT"
                    | "DUPLICATE_TARGET"
                    | "COMPILATION_DUPLICATE_SERIES"
                    | "COMPILATION_IDENTITY_COLLISION"
                    | "FINGERPRINT_CANONICALIZATION_LIMIT"
                    | "FINGERPRINT_SCHEMA_UNSUPPORTED"
            )
        }),
        "pre-Episode non-fingerprintable admission",
    )?;
    match &v.generation {
        DigestGenerationV1::NotAdmitted { proof_claim, .. } => {
            require(
                v.planner_admission.admission_class
                    == PlannerAdmissionClassToken::InsufficientInformation
                    && *proof_claim == ProofClaimToken::None
                    && v.candidates.is_empty(),
                "inconsistent NOT_ADMITTED observation",
            )?;
            require(
                matches!(
                    v.product_eligibility,
                    DigestProductEligibilityV1::NotApplicable {
                        reason: ProductDecisionReasonToken::PlanningNotAdmitted
                    }
                ) && matches!(
                    v.ranking_observation,
                    DigestRankingObservationV1::NotApplicable {
                        reason: ProductDecisionReasonToken::PlanningNotAdmitted
                    }
                ) && matches!(
                    v.display_decision,
                    DigestDisplayDecisionV1::NotApplicable {
                        reason: ProductDecisionReasonToken::PlanningNotAdmitted
                    }
                ),
                "NOT_ADMITTED requires downstream NOT_APPLICABLE",
            )?;
        }
        DigestGenerationV1::Searched {
            declared_hypotheses,
            visited_hypotheses,
            declared_hypothesis_count,
            evaluated_positions,
            discovered_candidate_count,
            declared_set_identity_digest,
            visited_set_identity_digest,
            visited_order_digest,
            outcome,
            search_assessment,
            search_exhausted,
            proof_claim,
        } => {
            require(
                v.planner_admission.admission_class
                    != PlannerAdmissionClassToken::InsufficientInformation,
                "SEARCHED requires admitted planning",
            )?;
            require(
                declared_hypothesis_count.value() == declared_hypotheses.len() as u64
                    && evaluated_positions.value() == visited_hypotheses.len() as u64
                    && discovered_candidate_count.value() == v.candidates.len() as u64,
                "search materialization count mismatch",
            )?;
            let declared = declared_hypotheses
                .iter()
                .map(CanonicalSearchHypothesisV0_1::canonical_bytes)
                .collect::<Result<Vec<_>>>()?;
            let visited = visited_hypotheses
                .iter()
                .map(CanonicalSearchHypothesisV0_1::canonical_bytes)
                .collect::<Result<Vec<_>>>()?;
            require(
                declared.windows(2).all(|p| p[0] < p[1]),
                "noncanonical declared hypotheses",
            )?;
            let x: BTreeSet<_> = declared.iter().collect();
            let s: BTreeSet<_> = visited.iter().collect();
            require(
                s.len() == visited.len() && s.is_subset(&x),
                "duplicate or undeclared search visit",
            )?;
            require(
                *declared_set_identity_digest == hypothesis_set_identity(declared_hypotheses)?
                    && *visited_set_identity_digest == hypothesis_set_identity(visited_hypotheses)?
                    && *visited_order_digest == visit_order_identity(visited_hypotheses)?,
                "search identity mismatch",
            )?;
            require(
                *search_exhausted == (s == x),
                "exhaustion disagrees with materialized sets",
            )?;
            let expected = if !search_exhausted {
                (SearchAssessmentToken::BestFound, ProofClaimToken::None)
            } else if v.candidates.is_empty() {
                (
                    SearchAssessmentToken::Complete,
                    ProofClaimToken::CompleteTraversal,
                )
            } else {
                (
                    SearchAssessmentToken::ProvenOptimal,
                    ProofClaimToken::ProvenOptimalDeterministicObjective,
                )
            };
            require(
                (*search_assessment, *proof_claim) == expected,
                "inconsistent search assessment or proof",
            )?;
            require(
                (*outcome == PlanningOutcomeToken::Solution) != v.candidates.is_empty(),
                "outcome disagrees with discovered candidates",
            )?;
            let mut fingerprints = BTreeSet::new();
            let mut placements = BTreeSet::new();
            for (index, c) in v.candidates.iter().enumerate() {
                require(
                    c.deterministic_rank_in_discovered.value() == index as u64 + 1,
                    "noncanonical discovered ranks",
                )?;
                require(
                    fingerprints.insert(&c.candidate_fingerprint),
                    "duplicate candidate fingerprint",
                )?;
                let h = CanonicalSearchHypothesisV0_1 {
                    schema_version: SearchHypothesisSchemaVersion::V0_1,
                    placements: c.placements.clone(),
                }
                .canonical_bytes()?;
                require(
                    placements.insert(h.clone()) && s.contains(&h),
                    "duplicate or unvisited candidate placement",
                )?;
                let f = &c.ranking_feature_snapshot;
                require(
                    f.schema_version.major.value() == 0 && f.schema_version.minor.value() == 1,
                    "unsupported ranking feature schema",
                )?;
                let distance = match (
                    &f.preferred_start_distance_seconds,
                    &f.preferred_start_source,
                ) {
                    (CanonicalOptionalV1::None {}, CanonicalOptionalV1::None {}) => 0,
                    (CanonicalOptionalV1::Some { value }, CanonicalOptionalV1::Some { .. }) => {
                        value.value()
                    }
                    _ => {
                        return Err(EvaluationContractError(
                            "inconsistent preferred-start option states",
                        ));
                    }
                };
                let key = &c.deterministic_ordering_key;
                require(
                    f.mutation_count.value() <= u64::from(u32::MAX)
                        && key.preference_distance_seconds.value() == distance
                        && key.mutation_count == f.mutation_count
                        && key.shift_seconds == f.shift_seconds
                        && key.start == c.placements[0].start
                        && key.object_id == c.placements[0].object_id,
                    "ordering key disagrees with candidate observations",
                )?;
            }
            // Check supplied ranks against the complete qualified tuple; never rerank.
            require(
                v.candidates
                    .windows(2)
                    .all(|p| p[0].deterministic_ordering_key <= p[1].deterministic_ordering_key),
                "rank order disagrees with deterministic keys",
            )?;
            let DigestProductEligibilityV1::Applied { records } = &v.product_eligibility else {
                return Err(EvaluationContractError(
                    "SEARCHED requires applied eligibility",
                ));
            };
            require(
                records.len() == v.candidates.len()
                    && records
                        .windows(2)
                        .all(|p| p[0].candidate_fingerprint < p[1].candidate_fingerprint),
                "noncanonical eligibility coverage",
            )?;
            let mut eligible = BTreeSet::new();
            for r in records {
                require(
                    fingerprints.contains(&r.candidate_fingerprint)
                        && r.eligible
                            == matches!(r.exclusion_rule_id, CanonicalOptionalV1::None {}),
                    "invalid eligibility record",
                )?;
                if r.eligible {
                    eligible.insert(&r.candidate_fingerprint);
                }
            }
            let DigestRankingObservationV1::Applied {
                ranked_eligible_order,
            } = &v.ranking_observation
            else {
                return Err(EvaluationContractError("SEARCHED requires applied ranking"));
            };
            let expected: Vec<_> = v
                .candidates
                .iter()
                .filter(|c| eligible.contains(&c.candidate_fingerprint))
                .map(|c| c.candidate_fingerprint.clone())
                .collect();
            require(
                *ranked_eligible_order == expected,
                "ranked eligible order disagrees with discovered ranks",
            )?;
            let DigestDisplayDecisionV1::Applied {
                display_policy_output_order,
            } = &v.display_decision
            else {
                return Err(EvaluationContractError("SEARCHED requires applied display"));
            };
            let l: BTreeSet<_> = display_policy_output_order.iter().collect();
            require(
                l.len() == display_policy_output_order.len() && l.is_subset(&eligible),
                "display output is not a unique eligible subset",
            )?;
        }
    }
    Ok(())
}

/// Production projection from native observations; no lifecycle/exposure data accepted.
#[derive(Debug, Clone)]
pub struct DecisionObservationProjectionV1 {
    payload: DecisionObservationDigestPayloadV1,
}
impl DecisionObservationProjectionV1 {
    pub fn from_observations(
        decision_input_fingerprint: SHA256Hex,
        generation: &CandidateGenerationObservationV0_1,
        candidates: &[CandidateObservationV0_1],
        eligibility: &ProductEligibilityObservationV0_1,
        ranking: &RankingObservationV0_1,
        display: &DisplayDecisionV0_1,
    ) -> Result<Self> {
        let mut ids = BTreeMap::new();
        let mut projected = Vec::new();
        for c in candidates {
            let fingerprint =
                CandidateFingerprintPayloadV1::new(c.placements[0].clone())?.fingerprint()?;
            require(
                fingerprint == c.candidate_fingerprint,
                "candidate fingerprint disagrees with placement",
            )?;
            require(
                ids.insert(c.candidate_id.clone(), fingerprint.clone())
                    .is_none(),
                "duplicate native CandidateId",
            )?;
            let f = &c.ranking_feature_snapshot;
            let key = &c.deterministic_ordering_key;
            projected.push(DigestCandidateV1 {
                candidate_fingerprint: fingerprint,
                placements: c.placements.clone(),
                ranking_feature_snapshot: DigestRankingFeatureSetV0_1 {
                    schema_version: DigestRankingFeatureSchemaV0_1 {
                        major: scenario::unsigned(0),
                        minor: scenario::unsigned(1),
                    },
                    preferred_start_distance_seconds: scenario::optional(
                        f.preferred_start_distance_seconds().map(scenario::unsigned),
                    ),
                    preferred_start_source: scenario::optional(
                        f.preferred_start_source().map(scenario::preference_source),
                    ),
                    mutation_count: scenario::unsigned(u64::from(f.mutation_count())),
                    shift_seconds: scenario::unsigned(f.shift_seconds()),
                },
                deterministic_rank_in_discovered: c.deterministic_rank_in_discovered,
                deterministic_ordering_key: DigestDeterministicOrderingKeyV1 {
                    preference_distance_seconds: scenario::unsigned(
                        key.preference_distance_seconds,
                    ),
                    mutation_count: scenario::unsigned(u64::from(key.mutation_count)),
                    shift_seconds: scenario::unsigned(key.shift_seconds),
                    start: key.start.try_into()?,
                    object_id: key.object_id.clone(),
                },
            });
        }
        projected.sort_by_key(|c| c.deterministic_rank_in_discovered);
        let resolve = |id: &CandidateId| {
            ids.get(id)
                .cloned()
                .ok_or(EvaluationContractError("unknown native CandidateId"))
        };
        let product_eligibility = match eligibility {
            ProductEligibilityObservationV0_1::NotApplicable { reason } => {
                DigestProductEligibilityV1::NotApplicable { reason: *reason }
            }
            ProductEligibilityObservationV0_1::Applied { records } => {
                let mut result = records
                    .iter()
                    .map(|r| {
                        Ok(DigestProductEligibilityRecordV1 {
                            candidate_fingerprint: resolve(&r.candidate_id)?,
                            eligible: r.eligible,
                            exclusion_rule_id: scenario::optional(r.exclusion_rule_id.clone()),
                        })
                    })
                    .collect::<Result<Vec<_>>>()?;
                result.sort_by(|a, b| a.candidate_fingerprint.cmp(&b.candidate_fingerprint));
                DigestProductEligibilityV1::Applied { records: result }
            }
        };
        let ranking_observation = match ranking {
            RankingObservationV0_1::NotApplicable { reason } => {
                DigestRankingObservationV1::NotApplicable { reason: *reason }
            }
            RankingObservationV0_1::Applied {
                ranked_eligible_order,
            } => DigestRankingObservationV1::Applied {
                ranked_eligible_order: ranked_eligible_order
                    .iter()
                    .map(&resolve)
                    .collect::<Result<_>>()?,
            },
        };
        let display_decision = match display {
            DisplayDecisionV0_1::NotApplicable { reason } => {
                DigestDisplayDecisionV1::NotApplicable { reason: *reason }
            }
            DisplayDecisionV0_1::Applied {
                display_policy_output_order,
            } => DigestDisplayDecisionV1::Applied {
                display_policy_output_order: display_policy_output_order
                    .iter()
                    .map(&resolve)
                    .collect::<Result<_>>()?,
            },
        };
        let (planner_admission, generation) = match generation {
            CandidateGenerationObservationV0_1::NotAdmitted {
                planner_admission,
                outcome,
            } => (
                planner_admission.clone(),
                DigestGenerationV1::NotAdmitted {
                    outcome: *outcome,
                    proof_claim: ProofClaimToken::None,
                },
            ),
            CandidateGenerationObservationV0_1::Searched {
                planner_admission,
                search_visit: s,
                outcome,
                search_assessment,
                search_exhausted,
                proof_claim,
            } => (
                planner_admission.clone(),
                DigestGenerationV1::Searched {
                    declared_hypotheses: s.declared_hypotheses.clone(),
                    visited_hypotheses: s.visited_hypotheses.clone(),
                    declared_hypothesis_count: s.declared_hypothesis_count,
                    evaluated_positions: s.evaluated_positions,
                    discovered_candidate_count: scenario::unsigned(candidates.len() as u64),
                    declared_set_identity_digest: s.declared_set_identity_digest.clone(),
                    visited_set_identity_digest: s.visited_set_identity_digest.clone(),
                    visited_order_digest: s.visited_order_digest.clone(),
                    outcome: *outcome,
                    search_assessment: *search_assessment,
                    search_exhausted: *search_exhausted,
                    proof_claim: *proof_claim,
                },
            ),
        };
        Ok(Self {
            payload: DecisionObservationDigestWireV1 {
                schema_version: DecisionObservationSchemaVersionV1::V1,
                decision_input_fingerprint,
                planner_admission,
                generation,
                candidates: projected,
                product_eligibility,
                ranking_observation,
                display_decision,
            }
            .try_into()?,
        })
    }
    pub fn payload(&self) -> &DecisionObservationDigestPayloadV1 {
        &self.payload
    }
}
