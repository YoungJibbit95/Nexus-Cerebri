#[path = "../../../tests/support/mod.rs"]
mod support;
use cerebri_planner::*;
use cerebri_temporal::*;
use cerebri_types::*;
use support::*;
#[path = "../../../tests/support/generator.rs"]
mod generator;

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
fn compilation_errors_never_fall_back_to_legacy_planning() {
    for mode in 0..6 {
        let mut input = input();
        let temporal = input.context.temporal.as_mut().unwrap();
        match mode {
            0 => temporal.limits.max_dates = 0,
            1 => temporal.series.push(temporal.series[0].clone()),
            2 => temporal.series[0].state = SeriesState::Prospective,
            3 => temporal.series[0].provenance = Provenance::ModelInference,
            4 => {
                temporal.horizon =
                    PlanningHorizon(range("2026-10-01T08:00:00Z", "2026-10-01T12:00:00Z"))
            }
            _ => temporal.series[0].id = SeriesId::new("busy").unwrap(),
        }
        let error = compile(&input).unwrap_err();
        let result = BaselinePlanner.plan(input.clone());
        assert!(
            result
                .validation
                .issues
                .contains(&ValidationIssue::Compilation(error))
        );
        assert_eq!(result.outcome, PlanningOutcome::InsufficientInformation);
        assert_eq!(result.assessment, SearchAssessment::BestFound);
        assert_eq!(result.search_space.evaluated, 0);
        assert!(result.candidates.is_empty());
        assert!(result.compilation.is_none());
        input.context.temporal = None;
        assert_eq!(
            BaselinePlanner.plan(input).outcome,
            PlanningOutcome::Solution
        );
    }
}

#[test]
fn nominal_identity_fields_are_sensitive_but_fold_resolution_is_not_identity() {
    let mut original = input();
    original.scope.time_range = range("2026-10-24T00:00:00Z", "2026-10-27T00:00:00Z");
    let source = original.context.temporal.as_mut().unwrap();
    source.horizon = PlanningHorizon(original.scope.time_range);
    let mut rule = source.series[0].rule.definition().clone();
    rule.start_date = "2026-10-25".parse().unwrap();
    rule.local_time = "02:30:00".parse().unwrap();
    rule.count = Some(1.try_into().unwrap());
    source.series[0].rule = RecurrenceRule::new(rule).unwrap();
    let first = compile(&original).unwrap().unwrap().occurrences.remove(0);
    for mode in 0..5 {
        let mut changed = original.clone();
        let series = &mut changed.context.temporal.as_mut().unwrap().series[0];
        let mut rule = series.rule.definition().clone();
        match mode {
            0 => series.id = SeriesId::new("other-series").unwrap(),
            1 => rule.start_date = "2026-10-26".parse().unwrap(),
            2 => rule.local_time = "02:31:00".parse().unwrap(),
            3 => rule.timezone = TimeZoneId::UTC,
            _ => rule.fold_policy = FoldPolicy::Later,
        }
        series.rule = RecurrenceRule::new(rule).unwrap();
        let next = compile(&changed).unwrap().unwrap().occurrences.remove(0);
        if mode == 4 {
            assert_eq!(next.id, first.id);
            assert_eq!(
                next.occurrence.range.start() - first.occurrence.range.start(),
                TimeDelta::hours(1)
            );
            assert_ne!(next.occurrence.resolution, first.occurrence.resolution);
        } else {
            assert_ne!(next.id, first.id, "identity field {mode}");
        }
    }
}

#[test]
fn seeded_daily_weekly_compilation_matches_day_by_day_oracle() {
    use generator::Generator;
    let anchor: Instant = "2026-10-05T00:00:00Z".parse().unwrap(); // Monday
    for seed in 0..256 {
        let mut rng = Generator(seed);
        let every = 1 + rng.pick(3);
        let weekly = rng.pick(2) == 0;
        let count = 1 + rng.pick(12);
        let until = rng.pick(28) as i64;
        let from = rng.pick(20) as i64;
        let to = from + 1 + rng.pick(9) as i64;
        let mut input = input();
        input.scope.time_range = TimeRange::new(
            anchor + TimeDelta::days(from) + TimeDelta::minutes(30),
            anchor + TimeDelta::days(to) + TimeDelta::minutes(30),
        )
        .unwrap();
        let t = input.context.temporal.as_mut().unwrap();
        t.horizon = PlanningHorizon(input.scope.time_range);
        t.series[0].rule = serde_json::from_value(serde_json::json!({
            "start_date":"2026-10-05", "local_time":"00:00:00", "timezone":"UTC", "duration":3600,
            "pattern": if weekly { serde_json::json!({"frequency":"WEEKLY", "every":every,"weekdays":["Mon","Wed","Sun"]}) } else { serde_json::json!({"frequency":"DAILY","every":every}) },
            "until":(anchor + TimeDelta::days(until)).date_naive().to_string(), "count":count, "gap_policy":"Reject", "fold_policy":"Reject"
        })).unwrap();
        // Enumerate nominal dates from the anchor; do not use sequence/seek/expand helpers.
        let nominal: Vec<i64> = (0..=until)
            .filter(|d| {
                if weekly {
                    (d / 7) % every as i64 == 0 && [0, 2, 6].contains(&(d % 7))
                } else {
                    d % every as i64 == 0
                }
            })
            .take(count as usize)
            .collect();
        let expected: Vec<_> = nominal
            .into_iter()
            .filter(|d| *d >= from && *d <= to)
            .map(|d| anchor + TimeDelta::days(d))
            .collect();
        let compiled = compile(&input).unwrap().unwrap();
        let mut actual: Vec<_> = compiled
            .occurrences
            .iter()
            .map(|o| o.occurrence.range.start())
            .collect();
        actual.sort();
        assert_eq!(
            actual,
            expected,
            "seed={seed}; request={}",
            serde_json::to_string(&input).unwrap()
        );
        for occurrence in &compiled.occurrences {
            assert_eq!(occurrence.occurrence.range.duration(), TimeDelta::hours(1));
            assert_eq!(
                occurrence.occurrence.visible_range.start(),
                occurrence
                    .occurrence
                    .range
                    .start()
                    .max(input.scope.time_range.start())
            );
            assert_eq!(
                occurrence.occurrence.visible_range.end(),
                occurrence
                    .occurrence
                    .range
                    .end()
                    .min(input.scope.time_range.end())
            );
        }
        // Shared date budget: enough for one expansion, insufficient for two identical rules.
        if seed < 16 && compiled.series[0].expansion.examined_dates > 0 {
            let used = compiled.series[0].expansion.examined_dates;
            let t = input.context.temporal.as_mut().unwrap();
            let mut second = t.series[0].clone();
            second.id = SeriesId::new("second").unwrap();
            t.series.push(second);
            t.limits.max_dates = used;
            assert!(
                matches!(
                    compile(&input),
                    Err(CompilationError::Temporal(TemporalError::DateLimitExceeded))
                ),
                "seed={seed}"
            );
        }
    }
}

#[test]
fn timezone_goldens_preserve_rejection_resolution_clipping_and_evidence() {
    for (zone, date, time, start, end, earlier, later, gap) in [
        (
            "Europe/Berlin",
            "2026-10-25",
            "02:30:00",
            "2026-10-25T00:00:00Z",
            "2026-10-25T04:00:00Z",
            "2026-10-25T00:30:00Z",
            "2026-10-25T01:30:00Z",
            false,
        ),
        (
            "Australia/Lord_Howe",
            "2026-04-05",
            "01:45:00",
            "2026-04-04T14:00:00Z",
            "2026-04-04T17:00:00Z",
            "2026-04-04T14:45:00Z",
            "2026-04-04T15:15:00Z",
            false,
        ),
        (
            "Europe/Berlin",
            "2026-03-29",
            "02:30:00",
            "2026-03-29T00:00:00Z",
            "2026-03-29T04:00:00Z",
            "",
            "",
            true,
        ),
        (
            "Pacific/Apia",
            "2011-12-30",
            "12:00:00",
            "2011-12-29T00:00:00Z",
            "2012-01-01T00:00:00Z",
            "",
            "",
            true,
        ),
    ] {
        let mut input = input();
        input.scope.time_range = range(start, end);
        let t = input.context.temporal.as_mut().unwrap();
        t.horizon = PlanningHorizon(input.scope.time_range);
        t.series[0].rule=serde_json::from_value(serde_json::json!({"start_date":date,"local_time":time,"timezone":zone,"duration":3600,"pattern":{"frequency":"DAILY","every":1},"count":1,"until":date,"gap_policy":"Reject","fold_policy":"Reject"})).unwrap();
        assert_eq!(
            compile(&input),
            Err(CompilationError::Temporal(if gap {
                TemporalError::NonexistentLocalTime
            } else {
                TemporalError::AmbiguousLocalTime
            }))
        );
        let mut identity = None;
        for policy in [FoldPolicy::Earlier, FoldPolicy::Later] {
            let t = input.context.temporal.as_mut().unwrap();
            let mut definition = t.series[0].rule.definition().clone();
            definition.fold_policy = policy;
            definition.gap_policy = GapPolicy::Skip;
            t.series[0].rule = RecurrenceRule::new(definition).unwrap();
            let compiled = compile(&input).unwrap().unwrap();
            if gap {
                assert!(compiled.occurrences.is_empty());
                assert_eq!(compiled.series[0].expansion.skipped.len(), 1);
                continue;
            }
            let o = &compiled.occurrences[0];
            assert_eq!(
                o.occurrence.range.start(),
                if policy == FoldPolicy::Earlier {
                    earlier
                } else {
                    later
                }
                .parse::<Instant>()
                .unwrap()
            );
            assert_eq!(o.evidence, vec![FactId::new("series-source").unwrap()]);
            if let Some(id) = &identity {
                assert_eq!(&o.id, id);
            } else {
                identity = Some(o.id.clone());
            }
            let mut clipped = input.clone();
            clipped.scope.time_range = TimeRange::new(
                o.occurrence.range.start() + TimeDelta::minutes(10),
                o.occurrence.range.end() - TimeDelta::minutes(10),
            )
            .unwrap();
            clipped.context.temporal.as_mut().unwrap().horizon =
                PlanningHorizon(clipped.scope.time_range);
            let c = compile(&clipped).unwrap().unwrap().occurrences.remove(0);
            assert_eq!(c.id, o.id);
            assert_eq!(c.occurrence.range, o.occurrence.range);
            assert_eq!(c.occurrence.visible_range, clipped.scope.time_range);
            assert_eq!(c.occurrence.resolution, o.occurrence.resolution);
            assert_eq!(c.evidence, o.evidence);
        }
    }
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
