#[test]
fn facts_and_preferences_cannot_override_hard_validity() {
    let mut input = request();
    input
        .preferences
        .preferences
        .push(cerebri_preferences::PreferenceEvidence {
            source: cerebri_preferences::PreferenceSource::PersonalLearned,
            preferred_start: "2026-10-01T09:00:00Z".parse().unwrap(),
            evidence: vec![],
        });
    let result = BaselinePlanner.plan(input.clone());
    assert_eq!(
        result.candidates[0].start.to_rfc3339(),
        "2026-10-01T10:00:00+00:00"
    );
    assert!(result.candidates[0].explanation.iter().any(|c| c.reason
        == PlanReason::PreferredStart(cerebri_preferences::PreferenceSource::PersonalLearned)));
    input.context.facts.push(Fact {
        id: FactId::new("contradiction").unwrap(),
        object_id: PlanningObjectId::new("busy").unwrap(),
        value: FactValue::ScheduledTime(range("2026-10-01T08:00:00Z", "2026-10-01T09:00:00Z")),
        provenance: Provenance::IntegrationFact,
    });
    assert!(
        validate_request(&input)
            .issues
            .contains(&ValidationIssue::ContradictoryFact(
                FactId::new("contradiction").unwrap()
            ))
    );
}
#[test]
fn external_lock_fact_blocks_a_permitted_move_and_move_cannot_resize() {
    let mut input = request();
    input.operation = Operation::Move;
    input.context.objects.remove(0);
    let object = input.context.objects[0].clone();
    input.target_ids = vec![object.id.clone()];
    input.planning_capability.mutations = vec![MutationGrant {
        object_id: object.id.clone(),
        calendar_id: object.calendar_id.clone(),
        integration_id: object.integration_id.clone(),
        kind: MutationKind::MoveEvent,
    }];
    let resized = ProposedPlan::from_placements(
        &input,
        vec![placement(
            "busy",
            "2026-10-01T10:00:00Z",
            "2026-10-01T10:30:00Z",
        )],
    );
    assert!(matches!(
        resized.validate(&input.context),
        Err(PlanError::Rejected(_))
    ));
    input.duration.value = FieldState::known(Duration::seconds(3600).unwrap());
    input.context.facts.push(Fact {
        id: FactId::new("locked").unwrap(),
        object_id: object.id,
        value: FactValue::ExternalLock,
        provenance: Provenance::IntegrationFact,
    });
    let moved = ProposedPlan::from_placements(
        &input,
        vec![placement(
            "busy",
            "2026-10-01T10:00:00Z",
            "2026-10-01T11:00:00Z",
        )],
    );
    assert!(matches!(
        moved.validate(&input.context),
        Err(PlanError::Rejected(_))
    ));
}
#[test]
fn forged_targets_out_of_horizon_and_mutation_limits_are_rejected() {
    let input = request();
    assert!(matches!(
        ProposedPlan::from_placements(&input, vec![]).validate(&input.context),
        Err(PlanError::Rejected(_))
    ));
    let outside = ProposedPlan::from_placements(
        &input,
        vec![placement(
            "new-event",
            "2026-10-01T12:00:00Z",
            "2026-10-01T12:30:00Z",
        )],
    );
    assert!(matches!(
        outside.validate(&input.context),
        Err(PlanError::Rejected(_))
    ));
    let mut input = request();
    input.policy.snapshot.mutation.max_mutations = 0;
    assert!(BaselinePlanner.plan(input).candidates.is_empty());
}
#[test]
fn budget_with_a_solution_is_still_best_found_and_exact_exhaustion_is_proven() {
    let mut input = request();
    input.budget.max_candidates = 5;
    let result = BaselinePlanner.plan(input.clone());
    assert_eq!(result.outcome, PlanningOutcome::Solution);
    assert_eq!(result.assessment, SearchAssessment::BestFound);
    input.budget.max_candidates = 11;
    assert_eq!(
        BaselinePlanner.plan(input).assessment,
        SearchAssessment::ProvenOptimal
    );
}
#[test]
fn combined_work_limit_rejects_expensive_search_before_evaluation() {
    let mut input = request();
    input.budget.max_candidates = 4096;
    let constraint = ConstraintSpec {
        object_id: input.target_ids[0].clone(),
        rule: HardConstraint::NoOverlap,
        evidence: vec![],
    };
    input.constraints = vec![constraint; 100];
    let result = BaselinePlanner.plan(input);
    assert_eq!(result.validation.issues, vec![ValidationIssue::InputLimit]);
    assert_eq!(result.search_space.evaluated, 0);
}
#[path = "../../../tests/support/mod.rs"]
mod support;
use cerebri_constraints::{ConstraintSpec, Fact, FactValue, HardConstraint};
use cerebri_planner::*;
use cerebri_temporal::Duration;
use cerebri_types::*;
use support::*;

#[test]
fn cpir_round_trip_and_future_schema_rejection() {
    let input = request();
    assert_eq!(
        serde_json::from_str::<PlanningRequest>(&serde_json::to_string(&input).unwrap()).unwrap(),
        input
    );
    let mut future = input;
    future.schema_version.minor = 2;
    assert!(
        validate_request(&future)
            .issues
            .contains(&ValidationIssue::UnsupportedSchema)
    );
    let mut json = serde_json::to_value(request()).unwrap();
    json["scope"]["max_mutations_typo"] = 0.into();
    assert!(serde_json::from_value::<PlanningRequest>(json).is_err());
}
#[test]
fn unresolved_required_time_blocks_planning() {
    for value in [
        FieldState::Unresolved,
        FieldState::Resolved(Knowledge::Unknown),
        FieldState::Resolved(Knowledge::Missing),
        FieldState::Resolved(Knowledge::Ambiguous(vec![])),
    ] {
        let mut input = request();
        input.context.objects[1].time.value = value;
        let result = BaselinePlanner.plan(input);
        assert_eq!(result.outcome, PlanningOutcome::InsufficientInformation);
        assert_eq!(result.search_space.evaluated, 0);
        assert!(result.candidates.is_empty());
    }
}
#[test]
fn uncertain_duration_requires_explicit_analysis_policy_and_is_explained() {
    let mut input = request();
    input.duration.value = FieldState::Resolved(Knowledge::Uncertain {
        value: Duration::seconds(1800).unwrap(),
        confidence: Confidence::new(0.5).unwrap(),
    });
    assert_eq!(
        validate_request(&input).state,
        ValidationState::InsufficientInformation
    );
    input.policy.snapshot.allow_uncertain_duration = true;
    assert_eq!(
        validate_request(&input).state,
        ValidationState::InsufficientInformation
    );
    input.operation = Operation::Analyze;
    let result = BaselinePlanner.plan(input);
    assert_eq!(
        result.validation.state,
        ValidationState::ValidWithUncertainty
    );
    assert!(
        result.candidates[0]
            .explanation
            .iter()
            .any(|s| s.reason == PlanReason::DurationUncertain)
    );
}
#[test]
fn scope_none_empty_and_explicit_lists_have_distinct_semantics() {
    let mut input = request();
    let target = input.context.objects[0].clone();
    assert!(input.scope.selects(&target));
    input.scope.object_ids = Some(vec![]);
    assert!(!input.scope.selects(&target));
    input.scope.object_ids = Some(vec![target.id.clone()]);
    assert!(input.scope.selects(&target));
    input.scope.object_ids = Some(vec![PlanningObjectId::new("other").unwrap()]);
    assert!(!input.scope.selects(&target));
    input.scope.object_ids = None;
    input.scope.calendar_ids = Some(vec![]);
    assert!(!input.scope.selects(&target));
    input.scope.calendar_ids = None;
    input.scope.integration_ids = Some(vec![]);
    assert!(!input.scope.selects(&target));
    input.scope.integration_ids = None;
    input.scope.resource_ids = Some(vec![]);
    assert!(!input.scope.selects(&target));
}
#[test]
fn empty_movable_scope_allows_no_moves() {
    let mut input = request();
    input.operation = Operation::Move;
    input.duration.value = FieldState::known(Duration::seconds(3600).unwrap());
    input.target_ids = vec![input.context.objects[1].id.clone()];
    input.context.objects.remove(0);
    let object = &input.context.objects[0];
    input.planning_capability.mutations = vec![MutationGrant {
        object_id: object.id.clone(),
        calendar_id: object.calendar_id.clone(),
        integration_id: object.integration_id.clone(),
        kind: MutationKind::MoveEvent,
    }];
    input.scope.movable_object_ids = Some(vec![]);
    let result = BaselinePlanner.plan(input.clone());
    assert!(result.candidates.is_empty());
    input.scope.movable_object_ids = None;
    assert!(!BaselinePlanner.plan(input).candidates.is_empty());
}
#[test]
fn analysis_only_scope_cannot_produce_mutating_action_plan() {
    let mut input = request();
    input.scope.max_mutations = 0;
    input.planning_capability.mutations.clear();
    let proposed = first_proposal(&input);
    assert_eq!(
        proposed
            .validate(&input.context)
            .unwrap()
            .into_action_plan()
            .unwrap_err(),
        PlanError::AnalysisOnly
    );
}
#[test]
fn read_visibility_and_inference_never_grant_mutation() {
    let mut input = request();
    input.planning_capability.mutations.clear();
    assert_eq!(
        BaselinePlanner.plan(input.clone()).outcome,
        PlanningOutcome::InsufficientInformation
    );
    input = request();
    input.planning_capability.plan = false;
    assert!(
        validate_request(&input)
            .issues
            .contains(&ValidationIssue::PlanningPermissionDenied)
    );
    input = request();
    input.context.facts.push(Fact {
        id: FactId::new("inferred").unwrap(),
        object_id: input.target_ids[0].clone(),
        value: FactValue::ExternalLock,
        provenance: Provenance::ModelInference,
    });
    assert!(
        validate_request(&input)
            .issues
            .contains(&ValidationIssue::InvalidFactSource(
                FactId::new("inferred").unwrap()
            ))
    );
}
#[test]
fn validator_rejects_hand_constructed_overlap_and_hard_constraint_violation() {
    let input = request();
    let bad = ProposedPlan::from_placements(
        &input,
        vec![placement(
            "new-event",
            "2026-10-01T09:30:00Z",
            "2026-10-01T10:00:00Z",
        )],
    );
    assert!(matches!(
        bad.validate(&input.context),
        Err(PlanError::Rejected(_))
    ));
    let mut input = request();
    input.constraints.push(ConstraintSpec {
        object_id: input.target_ids[0].clone(),
        rule: HardConstraint::LatestEnd("2026-10-01T09:00:00Z".parse().unwrap()),
        evidence: vec![],
    });
    let bad = ProposedPlan::from_placements(
        &input,
        vec![placement(
            "new-event",
            "2026-10-01T10:00:00Z",
            "2026-10-01T10:30:00Z",
        )],
    );
    assert!(matches!(
        bad.validate(&input.context),
        Err(PlanError::Rejected(_))
    ));
}
#[test]
fn stale_revision_or_changed_snapshot_requires_revalidation() {
    let input = request();
    let proposal = first_proposal(&input);
    let mut changed = input.context.clone();
    changed.revision.0 += 1;
    assert!(matches!(
        proposal.clone().validate(&changed),
        Err(PlanError::Stale)
    ));
    changed = input.context.clone();
    changed.objects[1].timezone = cerebri_temporal::TimeZoneId::UTC;
    assert!(matches!(proposal.validate(&changed), Err(PlanError::Stale)));
}
#[test]
fn search_assessment_never_claims_proof_for_partial_traversal() {
    let mut input = request();
    input.budget.max_candidates = 1;
    let limited = BaselinePlanner.plan(input);
    assert_eq!(limited.assessment, SearchAssessment::BestFound);
    assert_eq!(limited.outcome, PlanningOutcome::NeedsRelaxation);
    let mut input = request();
    input.scope.time_range = range("2026-10-01T09:00:00Z", "2026-10-01T10:00:00Z");
    let complete = BaselinePlanner.plan(input);
    assert_eq!(complete.assessment, SearchAssessment::Complete);
    assert_eq!(complete.outcome, PlanningOutcome::NoSolution);
    let optimal = BaselinePlanner.plan(request());
    assert_eq!(optimal.assessment, SearchAssessment::ProvenOptimal);
    assert_eq!(optimal.search_space.evaluated, 11);
    assert_eq!(optimal.candidates.len(), 7);
    assert_eq!(
        optimal.candidates[0].start.to_rfc3339(),
        "2026-10-01T10:00:00+00:00"
    );
}
#[test]
fn equivalent_inputs_have_stable_results_and_content_bound_plan_ids() {
    let input = request();
    let first = BaselinePlanner.plan(input.clone());
    let mut reordered = input.clone();
    reordered.context.objects.reverse();
    let second = BaselinePlanner.plan(reordered);
    assert_eq!(
        serde_json::to_value(&first).unwrap(),
        serde_json::to_value(second).unwrap()
    );
    assert_ne!(
        first.candidates[0].proposed.id(),
        first.candidates[1].proposed.id()
    );
    let mut changed = input;
    changed.policy.policy_version.0 += 1;
    assert_ne!(
        first.candidates[0].proposed.id(),
        first_proposal(&changed).id()
    );
}
#[test]
fn unsupported_intent_is_not_silently_translated_to_delete() {
    let mut input = request();
    input.operation = Operation::Cancel;
    assert!(
        validate_request(&input)
            .issues
            .contains(&ValidationIssue::UnsupportedOperation(Operation::Cancel))
    );
}
#[test]
fn authorization_requires_current_grants_and_confirmation_for_exact_plan() {
    let input = request();
    let plan = action_plan(&input);
    let mut current = authorization(&input, &plan);
    current.confirmed_plan_ids.clear();
    assert!(matches!(
        plan.authorize(&current),
        Err(PlanError::Unauthorized)
    ));
    let plan = action_plan(&input);
    current = authorization(&input, &plan);
    current.execution_grants.clear();
    assert!(matches!(
        plan.authorize(&current),
        Err(PlanError::Unauthorized)
    ));
}
