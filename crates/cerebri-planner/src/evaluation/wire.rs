//! Presence-aware component schemas. Cross-Episode lifecycle validation is not provided.
use super::RepositoryCommitHex;
use super::{
    EvaluationContractError, IdentifierString, LineageTransitionReasonToken, PlannerRunUseToken,
    required_nullable,
};
use cerebri_temporal::Instant;
use serde::{Deserialize, Serialize};

/// Strong Episode identity using the contract's bounded identifier grammar.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EpisodeId(IdentifierString);
impl EpisodeId {
    pub fn new(value: impl Into<String>) -> Result<Self, EvaluationContractError> {
        IdentifierString::new(value).map(Self)
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Sole run-identity/use component. Parent existence/equality require collection validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PlannerRunBindingV0_1 {
    planner_run_id: IdentifierString,
    run_use: PlannerRunUseToken,
    reused_from_episode_id: Option<EpisodeId>,
}
deserialize_object_via!(PlannerRunBindingV0_1, PlannerRunBindingWire);
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PlannerRunBindingWire {
    planner_run_id: IdentifierString,
    run_use: PlannerRunUseToken,
    #[serde(deserialize_with = "required_nullable")]
    reused_from_episode_id: Option<EpisodeId>,
}
impl PlannerRunBindingV0_1 {
    pub fn new(
        planner_run_id: IdentifierString,
        run_use: PlannerRunUseToken,
        reused_from_episode_id: Option<EpisodeId>,
    ) -> Result<Self, EvaluationContractError> {
        if (run_use == PlannerRunUseToken::ReusedRun) != reused_from_episode_id.is_some() {
            return Err(EvaluationContractError(
                "reused source is required exactly for REUSED_RUN",
            ));
        }
        Ok(Self {
            planner_run_id,
            run_use,
            reused_from_episode_id,
        })
    }
    pub fn planner_run_id(&self) -> &IdentifierString {
        &self.planner_run_id
    }
    pub fn run_use(&self) -> PlannerRunUseToken {
        self.run_use
    }
    pub fn reused_from_episode_id(&self) -> Option<&EpisodeId> {
        self.reused_from_episode_id.as_ref()
    }
}
impl TryFrom<PlannerRunBindingWire> for PlannerRunBindingV0_1 {
    type Error = EvaluationContractError;
    fn try_from(value: PlannerRunBindingWire) -> Result<Self, Self::Error> {
        Self::new(
            value.planner_run_id,
            value.run_use,
            value.reused_from_episode_id,
        )
    }
}

/// Immutable planner-run timing, separate from a later policy application's lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct PlannerRunTimestampsV0_1 {
    context_captured_at: Instant,
    generation_completed_at: Instant,
}
deserialize_object_via!(PlannerRunTimestampsV0_1, PlannerRunTimestampsWire);
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PlannerRunTimestampsWire {
    context_captured_at: Instant,
    generation_completed_at: Instant,
}
impl PlannerRunTimestampsV0_1 {
    pub fn new(
        context_captured_at: Instant,
        generation_completed_at: Instant,
    ) -> Result<Self, EvaluationContractError> {
        if context_captured_at > generation_completed_at {
            return Err(EvaluationContractError(
                "generation cannot precede context capture",
            ));
        }
        Ok(Self {
            context_captured_at,
            generation_completed_at,
        })
    }
    pub fn context_captured_at(self) -> Instant {
        self.context_captured_at
    }
    pub fn generation_completed_at(self) -> Instant {
        self.generation_completed_at
    }
}
impl TryFrom<PlannerRunTimestampsWire> for PlannerRunTimestampsV0_1 {
    type Error = EvaluationContractError;
    fn try_from(value: PlannerRunTimestampsWire) -> Result<Self, Self::Error> {
        Self::new(value.context_captured_at, value.generation_completed_at)
    }
}

closed_wire! {
/// Raw lifecycle timestamps only; Episode/run/event partial orders remain unvalidated.
/// This type deliberately grants no validated Episode status.
#[derive(Copy)]
pub struct EpisodeLifecycleTimestampsWireV0_1 {
    pub episode_started_at: Instant,
    #[serde(deserialize_with = "required_nullable")]
    pub policy_application_at: Option<Instant>,
    #[serde(deserialize_with = "required_nullable")]
    pub closed_at: Option<Instant>,
    pub episode_recorded_at: Instant,
}
}

closed_wire! {
/// Explicit orchestration observations; never inferred from hashes or revision changes.
#[derive(Copy)]
pub struct LineageTransitionCauseObservationV0_1 {
    pub context_changed: bool,
    pub explicit_replan: bool,
    pub new_visible_policy_application: bool,
}
}

closed_wire! {
/// Raw lineage component. Root/child reasons and parent/run consistency require later
/// Episode/collection validation; successful decoding alone grants neither.
pub struct EpisodeLineageWireV0_1 {
    pub family_id: IdentifierString,
    #[serde(deserialize_with = "required_nullable")]
    pub parent_episode_id: Option<EpisodeId>,
    #[serde(deserialize_with = "required_nullable")]
    pub transition_cause: Option<LineageTransitionCauseObservationV0_1>,
    pub transition_reasons: Vec<LineageTransitionReasonToken>,
}
}

closed_wire! {
/// Build/version metadata only (section 27); deliberately no planner artifact authority.
pub struct SoftwareProvenanceV0_1 {
    pub git_commit_sha: RepositoryCommitHex,
    pub workspace_version: IdentifierString,
    pub cpir_schema_version: IdentifierString,
    pub ranking_feature_schema_version: IdentifierString,
}
}

closed_wire! {
/// Reserved pseudonymous grouping shape; this type does not authorize human capture.
pub struct PrincipalGroupRef {
    pub group_id: IdentifierString,
    pub key_version: IdentifierString,
}
}

/// Synthetic privacy metadata, with content_fields_present fixed to false.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PrivacyV0_1 {
    classification: IdentifierString,
    retention_policy_id: IdentifierString,
    retention_policy_version: IdentifierString,
    content_fields_present: bool,
}
deserialize_object_via!(PrivacyV0_1, PrivacyWire);
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PrivacyWire {
    classification: IdentifierString,
    retention_policy_id: IdentifierString,
    retention_policy_version: IdentifierString,
    content_fields_present: bool,
}
impl PrivacyV0_1 {
    pub fn new(
        classification: IdentifierString,
        retention_policy_id: IdentifierString,
        retention_policy_version: IdentifierString,
    ) -> Self {
        Self {
            classification,
            retention_policy_id,
            retention_policy_version,
            content_fields_present: false,
        }
    }
    pub fn classification(&self) -> &IdentifierString {
        &self.classification
    }
    pub fn retention_policy_id(&self) -> &IdentifierString {
        &self.retention_policy_id
    }
    pub fn retention_policy_version(&self) -> &IdentifierString {
        &self.retention_policy_version
    }
}
impl TryFrom<PrivacyWire> for PrivacyV0_1 {
    type Error = EvaluationContractError;
    fn try_from(value: PrivacyWire) -> Result<Self, Self::Error> {
        if value.content_fields_present {
            return Err(EvaluationContractError(
                "E2 content_fields_present must be false",
            ));
        }
        Ok(Self::new(
            value.classification,
            value.retention_policy_id,
            value.retention_policy_version,
        ))
    }
}
