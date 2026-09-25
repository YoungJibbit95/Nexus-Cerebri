use crate::*;
use cerebri_temporal::{Duration, Instant, TimeRange};
use cerebri_types::PlanningObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlanningOutcome {
    Solution,
    NoSolution,
    NeedsRelaxation,
    InsufficientInformation,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchAssessment {
    ProvenOptimal,
    Complete,
    BestFound,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchSpace {
    pub horizon: TimeRange,
    pub granularity: Duration,
    pub objective: String,
    pub evaluated: u32,
    pub exhausted: bool,
}
#[derive(Debug, Clone, Serialize)]
pub struct RankedCandidate {
    pub proposed: ProposedPlan,
    pub cost: u64,
    pub mutation_count: u32,
    pub shifted_seconds: u64,
    pub start: Instant,
    pub object_id: PlanningObjectId,
    pub explanation: Vec<ScoreComponent>,
    pub ordering_key: CandidateOrderingKey,
    pub ranking_features: cerebri_preferences::RankingFeatureSet,
}
/// Lexicographic ranking; each component participates in precisely this order.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CandidateOrderingKey {
    pub preference_distance_seconds: u64,
    pub mutation_count: u32,
    pub shifted_seconds: u64,
    pub start: Instant,
    pub object_id: PlanningObjectId,
}
/// Sound rejected facts/constraints for this declared search space; never claimed minimal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConflictSet {
    pub rejections: Vec<CandidateRejection>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CandidateRejection {
    pub start: Instant,
    pub reasons: Vec<CandidateRejectionReason>,
}
#[derive(Debug, Clone, Serialize)]
pub struct PlanningResult {
    pub outcome: PlanningOutcome,
    pub assessment: SearchAssessment,
    pub validation: ValidationReport,
    pub candidates: Vec<RankedCandidate>,
    pub conflicts: ConflictSet,
    pub search_space: SearchSpace,
    pub compilation: Option<CompiledContextSnapshot>,
    pub dependency_graph: DependencyGraph,
}
pub trait Planner {
    fn plan(&self, request: PlanningRequest) -> PlanningResult;
}
#[derive(Default)]
pub struct BaselinePlanner;
impl Planner for BaselinePlanner {
    #[tracing::instrument(skip_all, fields(trace_id = %request.trace_id.as_str()))]
    fn plan(&self, request: PlanningRequest) -> PlanningResult {
        let (mut validation, compilation) = crate::validation::validate_and_compile(&request);
        // Baseline searches one event; lifecycle validation separately supports explicit batches.
        if request.target_ids.len() != 1
            && validation.state != ValidationState::InsufficientInformation
        {
            validation.state = ValidationState::InsufficientInformation;
            validation
                .issues
                .push(ValidationIssue::UnsupportedTargetCount);
        }
        let mut result = PlanningResult {
            outcome: PlanningOutcome::InsufficientInformation,
            assessment: SearchAssessment::BestFound,
            validation,
            dependency_graph: crate::dependency_graph(
                &request.context.objects,
                &request.constraints,
            ),
            compilation,
            candidates: vec![],
            conflicts: ConflictSet { rejections: vec![] },
            search_space: SearchSpace {
                horizon: request.scope.time_range,
                granularity: request.granularity,
                objective:
                    "preferred-start-distance-seconds; mutations; shift-seconds; start; object-id"
                        .into(),
                evaluated: 0,
                exhausted: false,
            },
        };
        if result.validation.state == ValidationState::InsufficientInformation {
            return result;
        }
        let request = std::sync::Arc::new(request);
        let object = request
            .object(&request.target_ids[0])
            .expect("validated target");
        let (duration, uncertain) = request
            .duration
            .value
            .required(request.analysis_only() && request.policy.snapshot.allow_uncertain_duration)
            .expect("validated duration");
        let mut start = request.scope.time_range.start();
        loop {
            let Ok(end) = duration.add_to(start) else {
                result.search_space.exhausted = true;
                break;
            };
            if end > request.scope.time_range.end() {
                result.search_space.exhausted = true;
                break;
            }
            if result.search_space.evaluated >= request.budget.max_candidates {
                break;
            }
            result.search_space.evaluated += 1;
            let placement = Placement {
                object_id: object.id.clone(),
                range: TimeRange::new(start, end).expect("positive duration"),
            };
            let reasons = crate::lifecycle::placement_violations_compiled(
                &request,
                std::slice::from_ref(&placement),
                result.compilation.as_ref(),
            );
            if reasons.is_empty() {
                let features = request.preferences.ranking_features(
                    start,
                    if request.analysis_only() { 0 } else { 1 },
                    object.time.value.required(false).ok().map(|(range, _)| range.start()),
                );
                let cost = features.preferred_start_distance_seconds().unwrap_or(0);
                let shift = features.shift_seconds();
                let mut explanation = vec![
                    ScoreComponent {
                        reason: PlanReason::FeasibleWithinScope,
                        cost: 0,
                    },
                    ScoreComponent {
                        reason: features.preferred_start_source().map_or(
                            PlanReason::EarliestTieBreak,
                            PlanReason::PreferredStart,
                        ),
                        cost,
                    },
                ];
                if request.analysis_only() {
                    explanation.push(ScoreComponent {
                        reason: PlanReason::AnalysisOnly,
                        cost: 0,
                    });
                }
                if uncertain {
                    explanation.push(ScoreComponent {
                        reason: PlanReason::DurationUncertain,
                        cost: 0,
                    });
                }
                result.candidates.push(RankedCandidate {
                    proposed: ProposedPlan::from_shared(request.clone(), vec![placement]),
                    cost,
                    mutation_count: features.mutation_count(),
                    shifted_seconds: shift,
                    start,
                    object_id: object.id.clone(),
                    explanation,
                    ordering_key: CandidateOrderingKey {
                        preference_distance_seconds: cost,
                        mutation_count: features.mutation_count(),
                        shifted_seconds: shift,
                        start,
                        object_id: object.id.clone(),
                    },
                    ranking_features: features,
                });
            } else {
                result
                    .conflicts
                    .rejections
                    .push(CandidateRejection { start, reasons });
            }
            match request.granularity.add_to(start) {
                Ok(next) => start = next,
                Err(_) => {
                    result.search_space.exhausted = true;
                    break;
                }
            }
        }
        result
            .candidates
            .sort_by(|a, b| a.ordering_key.cmp(&b.ordering_key));
        result.outcome = if !result.candidates.is_empty() {
            PlanningOutcome::Solution
        } else if result.search_space.exhausted {
            PlanningOutcome::NoSolution
        } else {
            PlanningOutcome::NeedsRelaxation
        };
        result.assessment = if !result.search_space.exhausted {
            SearchAssessment::BestFound
        } else if result.candidates.is_empty() {
            SearchAssessment::Complete
        } else {
            SearchAssessment::ProvenOptimal
        };
        result
    }
}
