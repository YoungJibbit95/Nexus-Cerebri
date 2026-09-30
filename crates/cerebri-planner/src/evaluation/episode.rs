//! Presence-aware Episode boundary. Phase A establishes wire validity, not R0/R3
//! verification, authenticated fingerprints, parent-collection validity or lifecycle reduction.
use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct EvaluationEpisodeSchemaVersionV0_1 {
    major: u16,
    minor: u16,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EpisodeSchemaWire {
    major: u16,
    minor: u16,
}
impl EvaluationEpisodeSchemaVersionV0_1 {
    pub const V0_1: Self = Self { major: 0, minor: 1 };
    pub fn major(self) -> u16 {
        self.major
    }
    pub fn minor(self) -> u16 {
        self.minor
    }
}
impl TryFrom<EpisodeSchemaWire> for EvaluationEpisodeSchemaVersionV0_1 {
    type Error = EvaluationContractError;
    fn try_from(value: EpisodeSchemaWire) -> Result<Self, Self::Error> {
        if (value.major, value.minor) != (0, 1) {
            return Err(EvaluationContractError("unsupported Episode schema"));
        }
        Ok(Self::V0_1)
    }
}
impl<'de> Deserialize<'de> for EvaluationEpisodeSchemaVersionV0_1 {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        object_only::<D, EpisodeSchemaWire>(deserializer)?
            .try_into()
            .map_err(serde::de::Error::custom)
    }
}

/// E2 has no non-null outcome constructor or wire variant (§41.2).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutcomeObservationV0_1 {}

closed_wire! {
/// The complete, closed Episode wire record. Required-nullable members never default.
/// Decoding this editable record alone does not grant validated domain status.
pub struct EvaluationEpisodeWireV0_1 {
    pub schema_version: EvaluationEpisodeSchemaVersionV0_1,
    pub episode_id: EpisodeId,
    #[serde(deserialize_with = "object_only")]
    pub lineage: EpisodeLineageWireV0_1,
    pub data_scope: DataScope,
    #[serde(deserialize_with = "required_nullable")]
    pub principal_group_ref: Option<PrincipalGroupRef>,
    #[serde(deserialize_with = "object_only")]
    pub planner_run_binding: PlannerRunBindingV0_1,
    #[serde(deserialize_with = "object_only")]
    pub planner_run_timestamps: PlannerRunTimestampsV0_1,
    #[serde(deserialize_with = "object_only")]
    pub lifecycle_timestamps: EpisodeLifecycleTimestampsWireV0_1,
    pub base_scenario_fingerprint: SHA256Hex,
    pub decision_input_fingerprint: SHA256Hex,
    pub decision_observation_digest: SHA256Hex,
    #[serde(deserialize_with = "object_only")]
    pub software_provenance: SoftwareProvenanceV0_1,
    pub replay_provenance: ReplayProvenanceV0_1,
    pub candidate_generation: CandidateGenerationObservationV0_1,
    pub candidates: Vec<CandidateObservationV0_1>,
    pub product_eligibility: ProductEligibilityObservationV0_1,
    pub ranking_observation: RankingObservationV0_1,
    pub display_decision: DisplayDecisionV0_1,
    pub exposure: ExposureObservationV0_1,
    pub manual_replacements: Vec<ManualReplacementObservationV0_1>,
    pub interaction_events: Vec<InteractionEventV0_1>,
    pub terminal_state: TerminalStateToken,
    pub capture_completeness: CaptureCompletenessV0_1,
    #[serde(deserialize_with = "required_nullable")]
    pub outcome: Option<OutcomeObservationV0_1>,
    #[serde(deserialize_with = "object_only")]
    pub privacy: PrivacyV0_1,
}
}

/// Wire-validated, immutable Episode domain container (Phase A).
///
/// This is NOT a replay/lifecycle proof. Phase C must add record-local semantic and
/// collection validation before Episodes can feed lifecycle or behavioral evaluation;
/// Phase B supplies fingerprint computation and Phase D authenticates replay evidence.
/// There is no production capture, execution or behavioral-evaluation consumer here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    try_from = "EvaluationEpisodeWireV0_1",
    into = "EvaluationEpisodeWireV0_1"
)]
pub struct EvaluationEpisodeV0_1(EvaluationEpisodeWireV0_1);
impl EvaluationEpisodeV0_1 {
    pub fn as_wire(&self) -> &EvaluationEpisodeWireV0_1 {
        &self.0
    }
    pub fn into_wire(self) -> EvaluationEpisodeWireV0_1 {
        self.0
    }
}
impl From<EvaluationEpisodeV0_1> for EvaluationEpisodeWireV0_1 {
    fn from(value: EvaluationEpisodeV0_1) -> Self {
        value.0
    }
}
impl TryFrom<EvaluationEpisodeWireV0_1> for EvaluationEpisodeV0_1 {
    type Error = EvaluationContractError;
    fn try_from(value: EvaluationEpisodeWireV0_1) -> Result<Self, Self::Error> {
        value.validate_foundations()?;
        Ok(Self(value))
    }
}

fn sorted_unique<'a>(tokens: impl Iterator<Item = &'a str>) -> bool {
    let mut previous = None;
    for token in tokens {
        if previous.is_some_and(|last| last >= token) {
            return false;
        }
        previous = Some(token);
    }
    true
}
fn placements_valid(placements: &[DigestPlacementV1]) -> bool {
    placements
        .iter()
        .all(|placement| placement.start < placement.end)
}

impl EvaluationEpisodeWireV0_1 {
    fn validate_foundations(&self) -> Result<(), EvaluationContractError> {
        let (admission, searched) = match &self.candidate_generation {
            CandidateGenerationObservationV0_1::NotAdmitted {
                planner_admission, ..
            } => (planner_admission, false),
            CandidateGenerationObservationV0_1::Searched {
                planner_admission,
                search_visit,
                ..
            } => {
                if !search_visit
                    .declared_hypotheses
                    .iter()
                    .chain(&search_visit.visited_hypotheses)
                    .all(|hypothesis| placements_valid(&hypothesis.placements))
                {
                    return Err(EvaluationContractError(
                        "invalid hypothesis placement extent",
                    ));
                }
                (planner_admission, true)
            }
        };
        if searched
            == (admission.admission_class == PlannerAdmissionClassToken::InsufficientInformation)
            || !sorted_unique(admission.issue_codes.iter().map(|code| code.as_str()))
        {
            return Err(EvaluationContractError(
                "invalid generation admission branch",
            ));
        }
        if !self
            .candidates
            .iter()
            .all(|candidate| placements_valid(&candidate.placements))
            || !self
                .manual_replacements
                .iter()
                .all(|replacement| placements_valid(&replacement.placements))
        {
            return Err(EvaluationContractError("invalid placement extent"));
        }
        if let ProductEligibilityObservationV0_1::Applied { records } = &self.product_eligibility
            && records
                .iter()
                .any(|record| record.eligible == record.exclusion_rule_id.is_some())
        {
            return Err(EvaluationContractError(
                "eligibility exclusion nullability mismatch",
            ));
        }
        if let CaptureCompletenessV0_1::Incomplete { reasons } = &self.capture_completeness
            && !sorted_unique(reasons.iter().map(|reason| reason.as_str()))
        {
            return Err(EvaluationContractError(
                "capture reasons must be unique and token-sorted",
            ));
        }
        self.validate_lineage_and_timestamps(searched)
    }

    fn validate_lineage_and_timestamps(
        &self,
        searched: bool,
    ) -> Result<(), EvaluationContractError> {
        let lineage = &self.lineage;
        let run = &self.planner_run_binding;
        if !sorted_unique(
            lineage
                .transition_reasons
                .iter()
                .map(|reason| reason.as_str()),
        ) || lineage.parent_episode_id.is_some() != lineage.transition_cause.is_some()
            || lineage.parent_episode_id.as_ref() == Some(&self.episode_id)
        {
            return Err(EvaluationContractError("invalid lineage structure"));
        }
        if lineage.parent_episode_id.is_none()
            && (run.run_use() != PlannerRunUseToken::NewRun
                || !lineage.transition_reasons.is_empty())
        {
            return Err(EvaluationContractError(
                "root must be a NEW_RUN without transition reasons",
            ));
        }
        if run.run_use() == PlannerRunUseToken::ReusedRun
            && run.reused_from_episode_id() != lineage.parent_episode_id.as_ref()
        {
            return Err(EvaluationContractError(
                "reused source must be the parent Episode",
            ));
        }
        let time = self.lifecycle_timestamps;
        let generated = self.planner_run_timestamps.generation_completed_at();
        if time.episode_started_at > time.episode_recorded_at
            || generated > time.episode_recorded_at
            || (run.run_use() == PlannerRunUseToken::NewRun
                && time.episode_started_at > self.planner_run_timestamps.context_captured_at())
            || searched != time.policy_application_at.is_some()
            || time
                .policy_application_at
                .is_some_and(|at| at < generated || at > time.episode_recorded_at)
            || time
                .closed_at
                .is_some_and(|at| at < time.episode_started_at || at > time.episode_recorded_at)
        {
            return Err(EvaluationContractError(
                "invalid Episode timestamp structure",
            ));
        }
        Ok(())
    }
}
