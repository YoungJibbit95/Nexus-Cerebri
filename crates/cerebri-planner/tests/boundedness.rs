#[path = "../../../tests/support/mod.rs"]
mod support;
use cerebri_constraints::{ConstraintSpec, HardConstraint};
use cerebri_planner::*;
use cerebri_temporal::*;
use cerebri_types::*;

#[test]
fn metadata_admission_accepts_exact_bounds_and_rejects_one_byte_more() {
    for (budget, size) in [(0, 256 * 1024), (4096, 4096)] {
        let mut input = support::request();
        input.context.objects.truncate(1);
        input.budget.max_candidates = budget;
        input.constraints = vec![ConstraintSpec {
            object_id: input.target_ids[0].clone(),
            rule: HardConstraint::ExternalLock {
                provenance: Provenance::SystemFact,
                reason: String::new(),
            },
            evidence: vec![],
        }];
        let padding = size - serde_json::to_vec(&input).unwrap().len();
        if let HardConstraint::ExternalLock { reason, .. } = &mut input.constraints[0].rule {
            *reason = "x".repeat(padding);
        }
        assert_eq!(serde_json::to_vec(&input).unwrap().len(), size);
        assert_eq!(validate_request(&input).state, ValidationState::Valid);
        if let HardConstraint::ExternalLock { reason, .. } = &mut input.constraints[0].rule {
            reason.push('x');
        }
        assert_eq!(
            validate_request(&input).issues,
            vec![ValidationIssue::InputLimit]
        );
    }
}

#[test]
fn repeated_evidence_work_is_admitted_before_search() {
    let mut input = support::request();
    input.context.objects.truncate(1);
    // The horizon must actually contain at least 256 evaluated placements.
    input.granularity = Duration::seconds(1).unwrap();
    input.constraints.push(ConstraintSpec {
        object_id: input.target_ids[0].clone(),
        rule: HardConstraint::EarliestStart(input.scope.time_range.end()),
        evidence: vec![FactId::new("x".repeat(128)).unwrap(); 1024],
    });
    input.budget.max_candidates = 256;
    // Fits HTTP input cap and old cardinality formula, but repeats > 32 MiB
    // of request metadata through per-candidate hashing/constraint cloning.
    assert!(serde_json::to_vec(&input).unwrap().len() < 256 * 1024);
    let result = BaselinePlanner.plan(input);
    assert!(
        result
            .validation
            .issues
            .contains(&ValidationIssue::InputLimit)
    );
    assert_eq!(result.search_space.evaluated, 0);
}

#[test]
fn serialized_input_is_bounded_even_for_direct_rust_callers() {
    let mut input = support::request();
    input.duration.evidence = vec![FactId::new("x".repeat(128)).unwrap(); 2048];
    input.budget.max_candidates = 0;
    assert_eq!(
        validate_request(&input).issues,
        vec![ValidationIssue::InputLimit]
    );
}

#[test]
fn supported_upper_bounds_terminate_without_wall_clock_assertions() {
    let mut input = support::request();
    input.context.objects.truncate(1);
    input.duration.value = FieldState::known(Duration::seconds(1).unwrap());
    input.granularity = Duration::seconds(1).unwrap();
    input.budget.max_candidates = 4096;
    let result = BaselinePlanner.plan(input.clone());
    assert_eq!(result.search_space.evaluated, 4096);
    assert_eq!(result.assessment, SearchAssessment::BestFound);
    input.budget.max_candidates = 1;
    input.constraints = vec![
        ConstraintSpec {
            object_id: input.target_ids[0].clone(),
            rule: HardConstraint::NoOverlap,
            evidence: vec![]
        };
        1024
    ];
    assert_eq!(
        BaselinePlanner.plan(input.clone()).search_space.evaluated,
        1
    );
    input.constraints.push(input.constraints[0].clone());
    assert!(
        validate_request(&input)
            .issues
            .contains(&ValidationIssue::InputLimit)
    );
    input.constraints.clear();
    input.budget.max_candidates = 0;
    let template = input.context.objects[0].clone();
    for i in 1..256 {
        let mut o = template.clone();
        o.id = PlanningObjectId::new(format!("resource-{i}")).unwrap();
        o.kind = PlanningObjectKind::Resource;
        input.context.objects.push(o);
    }
    assert_eq!(validate_request(&input).state, ValidationState::Valid);
}

#[test]
fn compiler_maximum_series_and_occurrences_and_evidence_amplification() {
    let mut input: PlanningRequest = serde_json::from_str(include_str!(
        "../../../examples/planner/recurrence-busy.json"
    ))
    .unwrap();
    let start = input.scope.time_range.start();
    input.scope.time_range = TimeRange::new(start, start + TimeDelta::days(32)).unwrap();
    let t = input.context.temporal.as_mut().unwrap();
    t.horizon = PlanningHorizon(input.scope.time_range);
    t.limits = ExpansionLimits {
        max_occurrences: 1024,
        max_dates: 36600,
    };
    let template = t.series[0].clone();
    t.series = (0..32)
        .map(|i| {
            let mut s = template.clone();
            s.id = SeriesId::new(format!("s-{i}")).unwrap();
            s
        })
        .collect();
    let compiled = compile_snapshot(&input.context, PlanningHorizon(input.scope.time_range))
        .unwrap()
        .unwrap();
    assert_eq!(compiled.occurrences.len(), 1024);
    let t = input.context.temporal.as_mut().unwrap();
    t.limits.max_occurrences = 1023;
    assert_eq!(
        compile_snapshot(&input.context, PlanningHorizon(input.scope.time_range)),
        Err(CompilationError::Temporal(
            TemporalError::OccurrenceLimitExceeded
        ))
    );
    let t = input.context.temporal.as_mut().unwrap();
    t.limits.max_occurrences = 1024;
    // Duplicate evidence is canonicalized away, so distinct IDs are required here.
    for series in &mut t.series {
        series.evidence = (0..1024)
            .map(|i| FactId::new(format!("{i:0128}")).unwrap())
            .collect();
    }
    assert!(matches!(
        compile_snapshot(&input.context, PlanningHorizon(input.scope.time_range)),
        Err(CompilationError::InputLimit)
    ));
}
