#[path = "../../../tests/support/mod.rs"]
mod support;
use cerebri_planner::*;
use cerebri_temporal::*;
use cerebri_types::*;
use support::*;

fn input() -> PlanningRequest {
    let mut input = request();
    input.schema_version = SchemaVersion::CPIR_0_2;
    input.context.temporal = Some(TemporalContext {
        horizon: PlanningHorizon(input.scope.time_range),
        coverage: Coverage::Complete,
        limits: ExpansionLimits {
            max_occurrences: 32,
            max_dates: 64,
        },
        series: vec![TemporalSeries {
            id: SeriesId::new("standup").unwrap(),
            state: SeriesState::Existing {
                revision: Revision(7),
            },
            provenance: Provenance::IntegrationFact,
            evidence: vec![FactId::new("series-source").unwrap()],
            rule: serde_json::from_value(serde_json::json!({
                "start_date":"2026-10-01", "local_time":"12:00:00", "timezone":"Europe/Berlin",
                "duration":3600, "pattern":{"frequency":"DAILY","every":1}, "until":null,
                "count":null,"gap_policy":"Reject","fold_policy":"Earlier"
            }))
            .unwrap(),
        }],
    });
    input
}
fn compile(input: &PlanningRequest) -> Result<Option<CompiledContextSnapshot>, CompilationError> {
    compile_snapshot(&input.context, PlanningHorizon(input.scope.time_range))
}

#[test]
fn occupied_occurrences_are_not_objects_or_mutations_and_revalidate() {
    let input = input();
    let compiled = compile(&input).unwrap().unwrap();
    assert_eq!(compiled.occurrences.len(), 1);
    let occurrence = &compiled.occurrences[0];
    assert_eq!(
        occurrence.occurrence.range,
        range("2026-10-01T10:00:00Z", "2026-10-01T11:00:00Z")
    );
    assert_eq!(occurrence.provenance, Provenance::IntegrationFact);
    assert_eq!(occurrence.source_revision, Revision(7));
    let result = BaselinePlanner.plan(input.clone());
    assert_eq!(
        result.candidates[0].start,
        "2026-10-01T11:00:00Z".parse::<Instant>().unwrap()
    );
    assert_eq!(result.compilation.as_ref(), Some(&compiled));
    assert!(result.conflicts.rejections.iter().flat_map(|r| &r.reasons).any(|reason|
        matches!(reason, CandidateRejectionReason::HardConstraint(v) if v.evidence.blocking_occurrences == vec![occurrence.id.clone()] && v.evidence.facts.contains(&FactId::new("series-source").unwrap()))));
    for candidate in result.candidates {
        let validated = candidate.proposed.validate(&input.context).unwrap();
        let actions = validated.into_action_plan().unwrap();
        assert_eq!(actions.actions().len(), 1);
        assert_eq!(actions.actions()[0].target.as_str(), "new-event");
    }
    let invalid = ProposedPlan::from_placements(
        &input,
        vec![placement(
            "new-event",
            "2026-10-01T10:00:00Z",
            "2026-10-01T10:30:00Z",
        )],
    );
    assert!(matches!(
        invalid.validate(&input.context),
        Err(PlanError::Rejected(_))
    ));
}

#[test]
fn identity_survives_horizon_clipping_revision_changes_and_recompilation() {
    let mut input = input();
    let full = compile(&input).unwrap().unwrap();
    input.scope.time_range = range("2026-10-01T10:30:00Z", "2026-10-01T12:00:00Z");
    let temporal = input.context.temporal.as_mut().unwrap();
    temporal.horizon = PlanningHorizon(input.scope.time_range);
    temporal.series[0].state = SeriesState::Existing {
        revision: Revision(8),
    };
    let clipped = compile(&input).unwrap().unwrap();
    assert_eq!(full.occurrences[0].id, clipped.occurrences[0].id);
    assert_eq!(
        full.occurrences[0].occurrence.range,
        clipped.occurrences[0].occurrence.range
    );
    assert_eq!(
        clipped.occurrences[0].occurrence.visible_range,
        range("2026-10-01T10:30:00Z", "2026-10-01T11:00:00Z")
    );
    assert_eq!(compile(&input).unwrap().unwrap(), clipped);
    input.scope.time_range = range("2026-10-01T11:00:00Z", "2026-10-01T12:00:00Z");
    input.context.temporal.as_mut().unwrap().horizon = PlanningHorizon(input.scope.time_range);
    assert!(compile(&input).unwrap().unwrap().occurrences.is_empty());
}

#[test]
fn duplicate_series_and_unsafe_sources_reject_atomically() {
    let original = input();
    let mut duplicate = original.clone();
    let series = duplicate.context.temporal.as_mut().unwrap();
    series.series.push(series.series[0].clone());
    assert!(matches!(
        compile(&duplicate),
        Err(CompilationError::DuplicateSeries(_))
    ));
    let mut collision = original.clone();
    collision.context.temporal.as_mut().unwrap().series[0].id = SeriesId::new("busy").unwrap();
    assert!(matches!(
        compile(&collision),
        Err(CompilationError::IdentityCollision(_))
    ));
    let mut prospective = original.clone();
    prospective.context.temporal.as_mut().unwrap().series[0].state = SeriesState::Prospective;
    assert!(matches!(
        compile(&prospective),
        Err(CompilationError::ProspectiveSeries(_))
    ));
    let mut inferred = original.clone();
    inferred.context.temporal.as_mut().unwrap().series[0].provenance = Provenance::ModelInference;
    assert!(matches!(
        compile(&inferred),
        Err(CompilationError::InvalidProvenance(_))
    ));
    for bad in [duplicate, collision, prospective, inferred] {
        let result = BaselinePlanner.plan(bad);
        assert_eq!(result.outcome, PlanningOutcome::InsufficientInformation);
        assert!(result.compilation.is_none());
        assert_eq!(result.search_space.evaluated, 0);
    }
}

#[test]
fn horizon_coverage_schema_and_shared_limits_fail_closed() {
    let original = input();
    let mut mismatch = original.clone();
    mismatch.scope.time_range = range("2026-10-01T10:00:00Z", "2026-10-01T11:00:00Z");
    assert_eq!(compile(&mismatch), Err(CompilationError::HorizonMismatch));
    let mut incomplete = original.clone();
    incomplete.context.temporal.as_mut().unwrap().coverage = Coverage::Incomplete;
    let result = BaselinePlanner.plan(incomplete);
    assert_eq!(result.outcome, PlanningOutcome::InsufficientInformation);
    let compiled = result.compilation.unwrap();
    assert!(compiled.availability.free.is_empty());
    assert!(!compiled.availability.unknown.is_empty());
    let mut old_schema = original.clone();
    old_schema.schema_version = SchemaVersion::CPIR_0_1;
    assert!(
        validate_request(&old_schema)
            .issues
            .contains(&ValidationIssue::UnsupportedSchema)
    );
    let mut limited = original.clone();
    let temporal = limited.context.temporal.as_mut().unwrap();
    temporal.limits.max_occurrences = 1;
    let mut second = temporal.series[0].clone();
    second.id = SeriesId::new("second").unwrap();
    temporal.series.push(second);
    assert_eq!(
        compile(&limited),
        Err(CompilationError::Temporal(
            TemporalError::OccurrenceLimitExceeded
        ))
    );
    limited.context.temporal.as_mut().unwrap().limits.max_dates = 0;
    assert_eq!(
        compile(&limited),
        Err(CompilationError::Temporal(TemporalError::DateLimitExceeded))
    );
    limited
        .context
        .temporal
        .as_mut()
        .unwrap()
        .limits
        .max_occurrences = 1_025;
    assert_eq!(compile(&limited), Err(CompilationError::InputLimit));
}

#[test]
fn canonical_compilation_ignores_collection_order() {
    let mut input = input();
    let temporal = input.context.temporal.as_mut().unwrap();
    let mut second = temporal.series[0].clone();
    second.id = SeriesId::new("second").unwrap();
    temporal.series.push(second);
    let expected = serde_json::to_value(BaselinePlanner.plan(input.clone())).unwrap();
    input.context.objects.reverse();
    input.context.facts.reverse();
    input.context.temporal.as_mut().unwrap().series.reverse();
    assert_eq!(
        serde_json::to_value(BaselinePlanner.plan(input)).unwrap(),
        expected
    );
}

#[test]
fn explicit_dst_resolution_and_skipped_dates_survive_compilation() {
    let mut input = input();
    for (date, fold, gap, expected, skipped) in [
        (
            "2026-10-25",
            "Earlier",
            "Reject",
            Some("2026-10-25T00:30:00Z"),
            false,
        ),
        (
            "2026-10-25",
            "Later",
            "Reject",
            Some("2026-10-25T01:30:00Z"),
            false,
        ),
        ("2026-03-29", "Reject", "Skip", None, true),
    ] {
        input.scope.time_range = range(&format!("{date}T00:00:00Z"), &format!("{date}T06:00:00Z"));
        let temporal = input.context.temporal.as_mut().unwrap();
        temporal.horizon = PlanningHorizon(input.scope.time_range);
        temporal.series[0].rule = serde_json::from_value(serde_json::json!({
            "start_date":date,"local_time":"02:30:00","timezone":"Europe/Berlin","duration":1800,
            "pattern":{"frequency":"DAILY","every":1},"until":date,"count":1,
            "fold_policy":fold,"gap_policy":gap
        }))
        .unwrap();
        let compiled = compile(&input).unwrap().unwrap();
        assert_eq!(
            compiled
                .occurrences
                .first()
                .map(|o| o.occurrence.range.start()),
            expected.map(|s| s.parse::<Instant>().unwrap())
        );
        assert_eq!(!compiled.series[0].expansion.skipped.is_empty(), skipped);
    }
}
