#[path = "../../../tests/support/generator.rs"]
mod generator;
#[path = "../../../tests/support/mod.rs"]
mod support;
use cerebri_constraints::{ConstraintSpec, Fact, FactValue, HardConstraint};
use cerebri_planner::*;
use cerebri_preferences::{PreferenceEvidence, PreferenceSource};
use cerebri_temporal::*;
use cerebri_types::*;
use generator::Generator;

fn at(minutes: i64) -> Instant {
    "2026-10-01T00:00:00Z".parse::<Instant>().unwrap() + TimeDelta::minutes(minutes)
}
fn range(start: i64, end: i64) -> TimeRange {
    TimeRange::new(at(start), at(end)).unwrap()
}
fn evidence() -> Vec<FactId> {
    ["source-b", "source-a"]
        .map(|s| FactId::new(s).unwrap())
        .to_vec()
}
fn constraint(rule: HardConstraint) -> ConstraintSpec {
    ConstraintSpec {
        object_id: PlanningObjectId::new("new-event").unwrap(),
        rule,
        evidence: evidence(),
    }
}

#[test]
fn ranking_observations_match_independent_millisecond_oracle_across_modes_and_permutations() {
    use PreferenceSource::*;
    for seed in 0..32_i64 {
        for fractional_ms in [0, 200, 800] {
            for mode in 0..4 {
                let mut input = support::request();
                let epoch = at(0) + TimeDelta::milliseconds(fractional_ms);
                let instant = |ms| epoch + TimeDelta::milliseconds(ms);
                let span = |a, b| TimeRange::new(instant(a), instant(b)).unwrap();
                input.scope.time_range = span(0, 8000);
                input.duration.value = FieldState::known(Duration::seconds(1).unwrap());
                input.granularity = Duration::seconds(1).unwrap();
                input.budget.max_candidates = if seed % 3 == 0 { 5 } else { 8 };
                input.context.objects[1].time.value = FieldState::known(span(3000, 4000));
                let original = if seed % 3 == 0 { None } else { Some([0, 800, 1000, 5200][seed as usize % 4]) };
                if let Some(original) = original {
                    input.context.objects[0].revision = Some(Revision(1));
                    input.context.objects[0].time.value = FieldState::known(span(original, original + 1000));
                    input.context.objects[0].time.provenance = Provenance::IntegrationFact;
                    input.operation = Operation::Move;
                    input.planning_capability.mutations[0].kind = MutationKind::MoveEvent;
                }
                match mode {
                    1 => input.operation = Operation::FindSlot,
                    2 => input.operation = Operation::Analyze,
                    3 => input.scope.max_mutations = 0,
                    _ => (),
                }
                input.constraints = vec![
                    constraint(HardConstraint::EarliestStart(instant(1000))),
                    constraint(HardConstraint::LatestEnd(instant(7000))),
                ];
                let source = [ExplicitCurrentRequest,SessionContext,PersonalLearned,GlobalLearned,Default][seed as usize % 5];
                let preferred = (seed % 9) * 800 - 800;
                if seed % 2 == 0 {
                    input.preferences.preferences = [preferred, preferred + 1000, preferred].map(|ms| PreferenceEvidence {
                        source, preferred_start: instant(ms), evidence: evidence(),
                    }).to_vec();
                    if source != Default {
                        input.preferences.preferences.push(PreferenceEvidence { source: Default, preferred_start: instant(0), evidence: vec![] });
                    }
                }
                let result = BaselinePlanner.plan(input.clone());
                assert_eq!(result.validation.state, ValidationState::Valid, "seed={seed}, mode={mode}");
                let mutations = if mode == 0 { 1 } else { 0 };
                // Independent integer oracle; no production feature/preference/ordering helper.
                let key = |ms: i64| (
                    if seed % 2 == 0 { ((ms - preferred) / 1000).unsigned_abs() } else { 0 },
                    mutations,
                    original.map_or(0, |original| ((original - ms) / 1000).unsigned_abs()),
                    ms,
                );
                let mut expected: Vec<i64> = (0..i64::from(input.budget.max_candidates))
                    .map(|s| s * 1000).filter(|s| *s >= 1000 && *s <= 6000 && *s != 3000).collect();
                expected.sort_by_key(|s| key(*s));
                assert_eq!(result.candidates.iter().map(|c| (c.start - epoch).num_milliseconds()).collect::<Vec<_>>(), expected);
                assert_eq!(result.search_space.evaluated, input.budget.max_candidates);
                assert_eq!(result.search_space.exhausted, input.budget.max_candidates == 8);
                assert_eq!(result.assessment, if input.budget.max_candidates == 8 { SearchAssessment::ProvenOptimal } else { SearchAssessment::BestFound });
                assert_eq!(result.outcome, PlanningOutcome::Solution);
                for candidate in &result.candidates {
                    let (distance, mutations, shift, _) = key((candidate.start - epoch).num_milliseconds());
                    let feature = &candidate.ranking_features;
                    assert_eq!(feature.preferred_start_distance_seconds(), (seed % 2 == 0).then_some(distance));
                    assert_eq!(feature.preferred_start_source(), (seed % 2 == 0).then_some(source));
                    assert_eq!(feature.mutation_count(), mutations);
                    assert_eq!(feature.shift_seconds(), shift);
                    assert_eq!(candidate.ordering_key, CandidateOrderingKey {
                        preference_distance_seconds: distance, mutation_count: mutations, shifted_seconds: shift,
                        start: candidate.start, object_id: input.target_ids[0].clone(),
                    });
                    assert_eq!((candidate.cost, candidate.mutation_count, candidate.shifted_seconds), (distance, mutations, shift));
                    candidate.proposed.clone().validate(&input.context).unwrap();
                }
                input.preferences.preferences.reverse();
                input.context.objects.reverse();
                input.constraints.reverse();
                assert_eq!(serde_json::to_value(&result).unwrap(), serde_json::to_value(BaselinePlanner.plan(input)).unwrap());
            }
        }
    }
}

#[test]
fn seeded_conjunction_oracle_and_complete_observable_permutation_campaign() {
    // Each seed creates one complete case, so replay does not depend on earlier cases.
    for seed in 0..512 {
        let mut rng = Generator(seed);
        let horizon = 4 + rng.pick(9) as i64;
        let step = 1 + rng.pick(3) as i64;
        let duration = 1 + rng.pick(5) as i64;
        let budget = rng.pick(15) as u32;
        let preferred = rng.pick((horizon + 2) as u64) as i64;
        let earliest = rng.pick((horizon + 1) as u64) as i64;
        let latest = 1 + rng.pick(horizon as u64) as i64;
        let deadline = 1 + rng.pick(horizon as u64) as i64;
        let buffer = rng.pick(3) as i64;
        let explicit = rng.pick(5) == 0;
        let exact_start = rng.pick(horizon as u64) as i64;
        let wrong_date = rng.pick(13) == 0;
        let locked = rng.pick(17) == 0;
        let dependency = rng.pick(2) == 0;
        let temporal = rng.pick(2) == 0;
        let incomplete = temporal && rng.pick(11) == 0;
        let mut input = support::request();
        input.scope.time_range = range(0, horizon);
        input.duration.value = FieldState::known(Duration::seconds(duration * 60).unwrap());
        input.granularity = Duration::seconds(step * 60).unwrap();
        input.budget.max_candidates = budget;
        input.context.objects[0].timezone = TimeZoneId::UTC;
        let prototype = input.context.objects[1].clone();
        input.context.objects.truncate(1);
        let mut busy = Vec::new();
        for i in 0..1 + rng.pick(3) {
            let start = rng.pick((horizon + 1) as u64) as i64;
            let end = start + 1 + rng.pick(3) as i64;
            busy.push((start, end));
            let mut object = prototype.clone();
            object.id = PlanningObjectId::new(format!("busy-{i}")).unwrap();
            object.time.value = FieldState::known(range(start, end));
            object.time.evidence = evidence();
            input.context.objects.push(object);
        }
        input.context.facts = vec![
            Fact {
                id: FactId::new("deadline").unwrap(),
                object_id: input.target_ids[0].clone(),
                value: FactValue::Deadline(Deadline(at(deadline))),
                provenance: Provenance::SystemFact,
            },
            Fact {
                id: FactId::new("busy-source").unwrap(),
                object_id: input.context.objects[1].id.clone(),
                value: FactValue::ScheduledTime(range(busy[0].0, busy[0].1)),
                provenance: Provenance::IntegrationFact,
            },
        ];
        input.constraints = vec![
            constraint(HardConstraint::EarliestStart(at(earliest))),
            constraint(HardConstraint::LatestEnd(at(latest))),
            constraint(HardConstraint::AvailabilityWindow(range(0, latest))),
            constraint(HardConstraint::FixedDuration(
                Duration::seconds(duration * 60).unwrap(),
            )),
            constraint(HardConstraint::MinDuration(Duration::seconds(60).unwrap())),
            constraint(HardConstraint::ExplicitDate(
                if wrong_date {
                    "2026-10-02"
                } else {
                    "2026-10-01"
                }
                .parse()
                .unwrap(),
            )),
            constraint(HardConstraint::TimezoneIntegrity(TimeZoneId::UTC)),
        ];
        if buffer > 0 {
            input
                .constraints
                .push(constraint(HardConstraint::RequiredBuffer(
                    Duration::seconds(buffer * 60).unwrap(),
                )));
        }
        if explicit {
            input
                .constraints
                .push(constraint(HardConstraint::ExplicitTime(range(
                    exact_start,
                    exact_start + duration,
                ))));
        }
        if locked {
            input.context.facts.push(Fact {
                id: FactId::new("lock").unwrap(),
                object_id: input.target_ids[0].clone(),
                value: FactValue::ExternalLock,
                provenance: Provenance::UserExplicit,
            });
        }
        if dependency {
            input
                .constraints
                .push(constraint(HardConstraint::DependencyOrder(
                    input.context.objects[1].id.clone(),
                )));
        }
        let predecessor_end = busy[0].1;
        if temporal {
            input.schema_version = SchemaVersion::CPIR_0_2;
            let template: PlanningRequest = serde_json::from_str(include_str!(
                "../../../examples/planner/recurrence-busy.json"
            ))
            .unwrap();
            let mut context = template.context.temporal.unwrap();
            context.horizon = PlanningHorizon(input.scope.time_range);
            context.coverage = if incomplete {
                Coverage::Incomplete
            } else {
                Coverage::Complete
            };
            let series = context.series[0].clone();
            context.series.clear();
            for i in 0..2 {
                let start = rng.pick((horizon + 1) as u64) as i64;
                let end = start + 1;
                busy.push((start, end));
                let mut item = series.clone();
                item.id = SeriesId::new(format!("series-{i}")).unwrap();
                item.evidence = evidence();
                let mut rule = item.rule.definition().clone();
                rule.timezone = TimeZoneId::UTC;
                rule.local_time = at(start).time();
                rule.duration = Duration::seconds(60).unwrap();
                rule.count = Some(1.try_into().unwrap());
                item.rule = RecurrenceRule::new(rule).unwrap();
                context.series.push(item);
            }
            input.context.temporal = Some(context);
        }
        input.preferences.preferences = vec![
            PreferenceEvidence {
                source: PreferenceSource::ExplicitCurrentRequest,
                preferred_start: at(preferred),
                evidence: evidence(),
            },
            PreferenceEvidence {
                source: PreferenceSource::PersonalLearned,
                preferred_start: at(earliest),
                evidence: evidence(),
            },
        ];
        let result = BaselinePlanner.plan(input.clone());
        let label = format!(
            "seed={seed}; request={}",
            serde_json::to_string(&input).unwrap()
        );
        // Independent integer-domain predicate: neither production constraint helpers,
        // temporal overlap functions nor lifecycle validation define this oracle.
        let feasible = |s: i64| {
            let e = s + duration;
            s >= earliest
                && e <= latest
                && e <= deadline
                && !wrong_date
                && !locked
                && (!explicit || s == exact_start)
                && (!dependency || s >= predecessor_end)
                && busy
                    .iter()
                    .all(|&(a, b)| e + buffer <= a || b + buffer <= s)
                && (!temporal || buffer == 0 || (s >= buffer && e + buffer <= horizon))
        };
        let grid: Vec<_> = (0..horizon)
            .filter(|s| s % step == 0 && s + duration <= horizon)
            .collect();
        let evaluated: Vec<_> = if incomplete {
            vec![]
        } else {
            grid.iter().copied().take(budget as usize).collect()
        };
        let mut expected: Vec<_> = evaluated.iter().copied().filter(|s| feasible(*s)).collect();
        expected.sort_by_key(|s| ((s - preferred).abs(), *s));
        let actual: Vec<_> = result
            .candidates
            .iter()
            .map(|c| (c.start - at(0)).num_minutes())
            .collect();
        assert_eq!(actual, expected, "{label}");
        let complete = !incomplete && evaluated.len() == grid.len();
        assert_eq!(result.search_space.exhausted, complete, "{label}");
        assert_eq!(
            result.search_space.evaluated as usize,
            evaluated.len(),
            "{label}"
        );
        assert_eq!(
            result.outcome,
            if incomplete {
                PlanningOutcome::InsufficientInformation
            } else if !expected.is_empty() {
                PlanningOutcome::Solution
            } else if complete {
                PlanningOutcome::NoSolution
            } else {
                PlanningOutcome::NeedsRelaxation
            },
            "{label}"
        );
        assert_eq!(
            result.assessment,
            if !complete {
                SearchAssessment::BestFound
            } else if expected.is_empty() {
                SearchAssessment::Complete
            } else {
                SearchAssessment::ProvenOptimal
            },
            "{label}"
        );
        let rejected: Vec<_> = evaluated
            .iter()
            .copied()
            .filter(|s| !feasible(*s))
            .collect();
        assert_eq!(
            result
                .conflicts
                .rejections
                .iter()
                .map(|r| (r.start - at(0)).num_minutes())
                .collect::<Vec<_>>(),
            rejected,
            "{label}"
        );
        for rejection in &result.conflicts.rejections {
            assert!(!rejection.reasons.is_empty(), "{label}");
        }
        for candidate in &result.candidates {
            assert_eq!(
                candidate.cost,
                (candidate.start - at(preferred))
                    .num_seconds()
                    .unsigned_abs(),
                "{label}"
            );
            candidate.proposed.clone().validate(&input.context).unwrap();
        }
        let canonical = serde_json::to_value(result).unwrap();
        for _ in 0..3 {
            rng.shuffle(&mut input.context.objects);
            rng.shuffle(&mut input.context.facts);
            rng.shuffle(&mut input.constraints);
            rng.shuffle(&mut input.preferences.preferences);
            for c in &mut input.constraints {
                rng.shuffle(&mut c.evidence);
            }
            for o in &mut input.context.objects {
                rng.shuffle(&mut o.time.evidence);
            }
            for p in &mut input.preferences.preferences {
                rng.shuffle(&mut p.evidence);
            }
            if let Some(t) = &mut input.context.temporal {
                rng.shuffle(&mut t.series);
                for s in &mut t.series {
                    rng.shuffle(&mut s.evidence);
                }
            }
            assert_eq!(
                serde_json::to_value(BaselinePlanner.plan(input.clone())).unwrap(),
                canonical,
                "permutation {label}"
            );
        }
    }
}

#[test]
fn arithmetic_boundaries_cannot_fabricate_search_proof() {
    for (start, end, duration, step, expected_count) in [
        (
            Instant::MAX_UTC - TimeDelta::seconds(2),
            Instant::MAX_UTC,
            1,
            i64::MAX / 1000,
            1,
        ),
        (
            Instant::MAX_UTC - TimeDelta::seconds(2),
            Instant::MAX_UTC,
            3,
            1,
            0,
        ),
        (
            Instant::MIN_UTC,
            Instant::MIN_UTC + TimeDelta::seconds(2),
            1,
            1,
            2,
        ),
    ] {
        let mut input = support::request();
        input.context.objects.truncate(1);
        input.scope.time_range = TimeRange::new(start, end).unwrap();
        input.duration.value = FieldState::known(Duration::seconds(duration).unwrap());
        input.granularity = Duration::seconds(step).unwrap();
        let result = BaselinePlanner.plan(input.clone());
        assert!(result.search_space.exhausted);
        assert_eq!(result.candidates.len(), expected_count);
        input.budget.max_candidates = 0;
        let partial = BaselinePlanner.plan(input);
        if expected_count > 0 {
            assert_eq!(partial.assessment, SearchAssessment::BestFound);
            assert_eq!(partial.outcome, PlanningOutcome::NeedsRelaxation);
        }
    }
}

#[test]
fn explicit_local_date_near_representable_instant_limits_does_not_panic() {
    for (start, end, zone, date) in [
        (
            Instant::MAX_UTC - TimeDelta::seconds(2),
            Instant::MAX_UTC,
            "Europe/Berlin",
            LocalDate::MAX,
        ),
        (
            Instant::MIN_UTC,
            Instant::MIN_UTC + TimeDelta::seconds(2),
            "America/New_York",
            LocalDate::MIN,
        ),
    ] {
        let mut input = support::request();
        input.context.objects.truncate(1);
        input.context.objects[0].timezone = zone.parse().unwrap();
        input.scope.time_range = TimeRange::new(start, end).unwrap();
        input.duration.value = FieldState::known(Duration::seconds(1).unwrap());
        input.granularity = Duration::seconds(1).unwrap();
        input
            .constraints
            .push(constraint(HardConstraint::ExplicitDate(date)));
        let wire = serde_json::to_string(&input).unwrap();
        let result =
            std::panic::catch_unwind(|| BaselinePlanner.plan(serde_json::from_str(&wire).unwrap()))
                .expect(
                    "externally supplied UTC instant must not panic during local date conversion",
                );
        assert_eq!(result.outcome, PlanningOutcome::NoSolution);
        assert!(result.candidates.is_empty());
    }
}
