use cerebri_temporal::*;
fn request() -> TemporalRequest {
    serde_json::from_str(include_str!("../../../examples/temporal-request.json")).unwrap()
}
#[test]
fn synthetic_diagnostics_are_reproducible_and_keep_gap_and_buffer_traces() {
    let result = temporal_diagnostics(request());
    assert_eq!(result, temporal_diagnostics(request()));
    let TemporalResult::Complete(report) = result else {
        panic!("expected complete diagnostics")
    };
    assert_eq!(report.expansions[0].skipped.len(), 1);
    assert_eq!(report.expansions[0].occurrences.len(), 2);
    assert_eq!(report.availability.busy.len(), 3);
    assert_eq!(report.availability.trace.len(), 3);
    assert_eq!(
        report.availability.trace[0]
            .buffered
            .duration()
            .num_seconds(),
        4800
    );
    assert!(report.availability.unknown.is_empty());
    let mut incomplete = request();
    incomplete.coverage = Coverage::Incomplete;
    let TemporalResult::Complete(incomplete) = temporal_diagnostics(incomplete) else {
        panic!("expected diagnostics")
    };
    assert_eq!(incomplete.availability.unknown, report.availability.free);
    assert!(incomplete.availability.free.is_empty());
}
#[test]
fn budgets_are_shared_across_rules_and_never_return_partial_availability() {
    let mut input = request();
    input.limits.max_occurrences = 2;
    input.recurrences.push(input.recurrences[0].clone());
    assert_eq!(
        temporal_diagnostics(input),
        TemporalResult::Rejected(TemporalError::OccurrenceLimitExceeded)
    );
    let mut input = request();
    input.limits.max_dates = 0;
    assert_eq!(
        temporal_diagnostics(input),
        TemporalResult::Rejected(TemporalError::DateLimitExceeded)
    );
    let mut input = request();
    input.recurrences = vec![input.recurrences[0].clone(); 33];
    assert_eq!(
        temporal_diagnostics(input),
        TemporalResult::Rejected(TemporalError::InputLimit)
    );
}
#[test]
fn all_wire_fields_are_explicit_and_unknown_fields_rejected() {
    let fixture = serde_json::to_value(request()).unwrap();
    for field in ["coverage", "horizon", "limits", "recurrences", "busy"] {
        let mut input = fixture.clone();
        input.as_object_mut().unwrap().remove(field);
        assert!(serde_json::from_value::<TemporalRequest>(input).is_err());
    }
    for field in [
        "gap_policy",
        "fold_policy",
        "timezone",
        "pattern",
        "duration",
    ] {
        let mut input = fixture.clone();
        input["recurrences"][0]
            .as_object_mut()
            .unwrap()
            .remove(field);
        assert!(serde_json::from_value::<TemporalRequest>(input).is_err());
    }
    let mut input = fixture;
    input["silent_default"] = true.into();
    assert!(serde_json::from_value::<TemporalRequest>(input).is_err());
}
