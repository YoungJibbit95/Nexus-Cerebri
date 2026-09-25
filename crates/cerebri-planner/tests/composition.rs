#[path = "../../../tests/support/mod.rs"]
mod support;
use cerebri_constraints::{ConstraintSpec, HardConstraint};
use cerebri_planner::*;
use cerebri_preferences::{PreferenceEvidence, PreferenceSource};
use cerebri_temporal::*;
use cerebri_types::*;
use support::*;

fn constraint(rule: HardConstraint) -> ConstraintSpec {
    ConstraintSpec {
        object_id: PlanningObjectId::new("new-event").unwrap(),
        rule,
        evidence: vec![],
    }
}
fn at(time: &str) -> Instant {
    format!("2026-10-01T{time}:00Z").parse().unwrap()
}

#[test]
fn complexity_manifest_oracles_and_legacy_full_order_regression() {
    let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples");
    let manifest: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(directory.join("manifest.json")).unwrap())
            .unwrap();
    // Recorded baseline at 6aae6d9; separate from the unchanged, independently
    // constructed manifest oracles. These snapshots are compatibility evidence.
    let legacy = [
        (
            11,
            true,
            vec![
                "10:00", "10:15", "10:30", "10:45", "11:00", "11:15", "11:30",
            ],
        ),
        (
            11,
            true,
            vec![
                "10:00", "10:15", "10:30", "10:45", "11:00", "11:15", "11:30",
            ],
        ),
        (
            11,
            true,
            vec![
                "10:45", "10:30", "11:00", "10:15", "11:15", "10:00", "11:30",
            ],
        ),
        (2, true, vec!["10:00"]),
        (2, true, vec![]),
        (15, true, vec!["11:00", "09:45", "12:15"]),
        (14, true, vec!["11:15", "11:00", "10:45"]),
        (
            11,
            true,
            vec!["10:15", "10:30", "10:45", "11:00", "11:15", "11:30"],
        ),
        (0, false, vec![]),
        (3, false, vec!["09:30", "09:15", "09:00"]),
        (18, true, vec!["12:15"]),
    ];
    let cases = manifest["scenarios"].as_array().unwrap();
    assert_eq!(cases.len(), legacy.len());
    for (case, (evaluated, exhausted, order)) in cases.iter().zip(legacy) {
        let input: PlanningRequest = serde_json::from_str(
            &std::fs::read_to_string(directory.join(case["file"].as_str().unwrap())).unwrap(),
        )
        .unwrap();
        let result = BaselinePlanner.plan(input.clone());
        let wire = serde_json::to_value(&result).unwrap();
        assert_eq!(wire["validation"]["state"], case["expected_validation"]);
        assert_eq!(wire["outcome"], case["expected_outcome"]);
        assert_eq!(wire["assessment"], case["expected_assessment"]);
        if let Some(count) = case.get("expected_candidates") {
            assert_eq!(result.candidates.len() as u64, count.as_u64().unwrap());
        }
        if let Some(start) = case.get("expected_first_start") {
            assert_eq!(wire["candidates"][0]["start"], *start);
        }
        assert_eq!(result.search_space.evaluated, evaluated, "{}", case["file"]);
        assert_eq!(result.search_space.exhausted, exhausted, "{}", case["file"]);
        assert_eq!(
            result
                .candidates
                .iter()
                .map(|c| c.start)
                .collect::<Vec<_>>(),
            order.into_iter().map(at).collect::<Vec<_>>(),
            "{}",
            case["file"]
        );
        for candidate in result.candidates {
            candidate.proposed.validate(&input.context).unwrap();
        }
    }
}

#[test]
fn all_hard_constraints_compose_with_compiled_occupancy() {
    let mut input: PlanningRequest = serde_json::from_str(include_str!(
        "../../../examples/planner/recurrence-busy.json"
    ))
    .unwrap();
    let temporal = input.context.temporal.as_mut().unwrap();
    let mut definition = temporal.series[0].rule.definition().clone();
    definition.duration = Duration::seconds(1800).unwrap();
    temporal.series[0].rule = RecurrenceRule::new(definition).unwrap();
    let selected = range("2026-10-01T10:45:00Z", "2026-10-01T11:15:00Z");
    input.constraints = vec![
        HardConstraint::NoOverlap,
        HardConstraint::ExplicitTime(selected),
        HardConstraint::ExplicitDate("2026-10-01".parse().unwrap()),
        HardConstraint::EarliestStart(at("10:30")),
        HardConstraint::LatestEnd(at("11:30")),
        HardConstraint::Deadline(Deadline(at("11:15"))),
        HardConstraint::MinDuration(Duration::seconds(1800).unwrap()),
        HardConstraint::FixedDuration(Duration::seconds(1800).unwrap()),
        HardConstraint::AvailabilityWindow(range("2026-10-01T10:30:00Z", "2026-10-01T11:45:00Z")),
        HardConstraint::DependencyOrder(PlanningObjectId::new("busy").unwrap()),
        HardConstraint::RequiredBuffer(Duration::seconds(900).unwrap()),
        HardConstraint::TimezoneIntegrity("Europe/Berlin".parse().unwrap()),
    ]
    .into_iter()
    .map(constraint)
    .collect();
    let result = BaselinePlanner.plan(input.clone());
    assert_eq!(result.outcome, PlanningOutcome::Solution);
    assert_eq!(result.candidates.len(), 1);
    assert_eq!(
        result.candidates[0].proposed.placements()[0].range,
        selected
    );
    result.candidates[0]
        .proposed
        .clone()
        .validate(&input.context)
        .unwrap();
    for contradiction in [
        HardConstraint::ExplicitTime(range("2026-10-01T11:00:00Z", "2026-10-01T11:30:00Z")),
        HardConstraint::ExplicitDate("2026-10-02".parse().unwrap()),
        HardConstraint::EarliestStart(at("11:00")),
        HardConstraint::LatestEnd(at("11:00")),
        HardConstraint::Deadline(Deadline(at("11:00"))),
        HardConstraint::MinDuration(Duration::seconds(3600).unwrap()),
        HardConstraint::FixedDuration(Duration::seconds(3600).unwrap()),
        HardConstraint::AvailabilityWindow(range("2026-10-01T09:00:00Z", "2026-10-01T10:00:00Z")),
        HardConstraint::RequiredBuffer(Duration::seconds(1800).unwrap()),
        HardConstraint::TimezoneIntegrity(TimeZoneId::UTC),
        HardConstraint::ExternalLock {
            provenance: Provenance::UserExplicit,
            reason: "must not change".into(),
        },
        HardConstraint::RecurrenceRule,
    ] {
        let mut rejected = input.clone();
        rejected.constraints.push(constraint(contradiction.clone()));
        let result = BaselinePlanner.plan(rejected.clone());
        assert_eq!(
            result.outcome,
            PlanningOutcome::NoSolution,
            "{contradiction:?}"
        );
        assert_eq!(result.assessment, SearchAssessment::Complete);
        assert!(matches!(
            ProposedPlan::from_placements(
                &rejected,
                vec![Placement {
                    object_id: rejected.target_ids[0].clone(),
                    range: selected
                }]
            )
            .validate(&rejected.context),
            Err(PlanError::Rejected(_))
        ));
    }
    // Same contracts and evidence must have identical output in another collection order.
    let expected = serde_json::to_value(BaselinePlanner.plan(input.clone())).unwrap();
    input.constraints.reverse();
    input.context.objects.reverse();
    assert_eq!(
        expected,
        serde_json::to_value(BaselinePlanner.plan(input)).unwrap()
    );
}

#[test]
fn golden_preference_order_exposes_complete_tuple_and_explicit_precedence() {
    let mut input = request();
    input.preferences.preferences = vec![
        PreferenceEvidence {
            source: PreferenceSource::PersonalLearned,
            preferred_start: at("10:00"),
            evidence: vec![],
        },
        PreferenceEvidence {
            source: PreferenceSource::ExplicitCurrentRequest,
            preferred_start: at("11:00"),
            evidence: vec![],
        },
    ];
    let result = BaselinePlanner.plan(input.clone());
    let expected = [
        ("11:00", 0),
        ("10:45", 900),
        ("11:15", 900),
        ("10:30", 1800),
        ("11:30", 1800),
        ("10:15", 2700),
        ("10:00", 3600),
    ];
    assert_eq!(result.candidates.len(), expected.len());
    for (candidate, (start, cost)) in result.candidates.iter().zip(expected) {
        assert_eq!(candidate.start, at(start));
        assert_eq!(candidate.cost, cost);
        assert_eq!(
            candidate
                .ranking_features
                .preferred_start_distance_seconds(),
            Some(cost)
        );
        assert_eq!(
            candidate.ranking_features.preferred_start_source(),
            Some(PreferenceSource::ExplicitCurrentRequest)
        );
        assert_eq!(candidate.ranking_features.mutation_count(), 1);
        assert_eq!(candidate.ranking_features.shift_seconds(), 0);
        assert_eq!(
            candidate.ordering_key,
            CandidateOrderingKey {
                preference_distance_seconds: cost,
                mutation_count: 1,
                shifted_seconds: 0,
                start: at(start),
                object_id: input.target_ids[0].clone(),
            }
        );
        assert!(candidate.explanation.iter().any(|s| s.reason
            == PlanReason::PreferredStart(PreferenceSource::ExplicitCurrentRequest)
            && s.cost == cost));
    }
    input.preferences.preferences.reverse();
    assert_eq!(
        serde_json::to_value(&result).unwrap(),
        serde_json::to_value(BaselinePlanner.plan(input.clone())).unwrap()
    );
    input.budget.max_candidates = 5;
    let partial = BaselinePlanner.plan(input);
    assert_eq!(partial.assessment, SearchAssessment::BestFound);
    assert_eq!(partial.candidates.len(), 1);
    assert_eq!(partial.candidates[0].start, at("10:00"));
}

#[test]
fn golden_shift_tiebreak_outranks_earliest_start() {
    let mut input = request();
    input.operation = Operation::FindSlot;
    input.context.objects.remove(0);
    input.context.objects[0].time.value =
        FieldState::known(range("2026-10-01T11:00:00Z", "2026-10-01T12:00:00Z"));
    input.target_ids = vec![input.context.objects[0].id.clone()];
    input.duration.value = FieldState::known(Duration::seconds(3600).unwrap());
    input.preferences.preferences = vec![PreferenceEvidence {
        source: PreferenceSource::ExplicitCurrentRequest,
        preferred_start: at("10:30"),
        evidence: vec![],
    }];
    let result = BaselinePlanner.plan(input);
    assert_eq!(
        result
            .candidates
            .iter()
            .take(5)
            .map(|c| c.start)
            .collect::<Vec<_>>(),
        ["10:30", "10:45", "10:15", "11:00", "10:00"].map(at)
    );
    assert_eq!(result.candidates[1].ordering_key.shifted_seconds, 900);
    assert_eq!(result.candidates[2].ordering_key.shifted_seconds, 2700);
}

#[test]
fn independent_integer_oracle_covers_small_schedule_combinations() {
    // Independent integer arithmetic oracle; no constraint checker or temporal overlap helper.
    let epoch = at("09:00");
    for busy_start in 0..5 {
        for busy_end in busy_start + 1..=6 {
            for duration in 1..=3 {
                let mut input = request();
                input.scope.time_range =
                    TimeRange::new(epoch, epoch + TimeDelta::minutes(6)).unwrap();
                input.granularity = Duration::seconds(60).unwrap();
                input.duration.value = FieldState::known(Duration::seconds(duration * 60).unwrap());
                input.context.objects[1].time.value = FieldState::known(
                    TimeRange::new(
                        epoch + TimeDelta::minutes(busy_start),
                        epoch + TimeDelta::minutes(busy_end),
                    )
                    .unwrap(),
                );
                input.constraints = vec![
                    constraint(HardConstraint::EarliestStart(epoch + TimeDelta::minutes(1))),
                    constraint(HardConstraint::LatestEnd(epoch + TimeDelta::minutes(5))),
                ];
                let result = BaselinePlanner.plan(input.clone());
                let expected: Vec<_> = (0..=6 - duration)
                    .filter(|s| {
                        *s >= 1
                            && s + duration <= 5
                            && (*s >= busy_end || s + duration <= busy_start)
                    })
                    .collect();
                let actual: Vec<_> = result
                    .candidates
                    .iter()
                    .map(|c| (c.start - epoch).num_minutes())
                    .collect();
                assert_eq!(
                    actual, expected,
                    "busy [{busy_start},{busy_end}), duration {duration}"
                );
                for candidate in result.candidates {
                    candidate.proposed.validate(&input.context).unwrap();
                }
            }
        }
    }
}

#[test]
fn recurrence_buffer_cannot_rely_on_unobserved_time_outside_horizon() {
    let mut input: PlanningRequest = serde_json::from_str(include_str!(
        "../../../examples/planner/recurrence-busy.json"
    ))
    .unwrap();
    input.context.objects.remove(1);
    input.context.temporal.as_mut().unwrap().series.clear();
    input
        .constraints
        .push(constraint(HardConstraint::RequiredBuffer(
            Duration::seconds(900).unwrap(),
        )));
    let result = BaselinePlanner.plan(input);
    assert_eq!(result.candidates.first().unwrap().start, at("09:15"));
    assert_eq!(result.candidates.last().unwrap().start, at("11:15"));
}
