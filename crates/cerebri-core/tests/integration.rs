use cerebri_core::integration::*;
use cerebri_planner::{PlanError, PlanningOutcome, SearchAssessment};
use serde_json::{Value, json};

const FIXTURE: &[u8] = include_bytes!("../../../examples/integration/suggestion.json");

fn value() -> Value {
    serde_json::from_slice(FIXTURE).unwrap()
}
fn run(value: &Value) -> SuggestionResponse {
    suggest_from_slice(&serde_json::to_vec(value).unwrap())
}

#[test]
fn unchanged_core_result_has_exact_starts_and_cannot_become_an_action() {
    let input: SuggestionRequest = serde_json::from_slice(FIXTURE).unwrap();
    let source = input.request.clone();
    let SuggestionOutcome::Planned {
        request_id,
        trace_id,
        context_revision,
        result,
    } = suggest(input).outcome
    else {
        panic!("synthetic profile rejected");
    };
    assert_eq!(request_id, source.request_id);
    assert_eq!(trace_id, source.trace_id);
    assert_eq!(context_revision, source.context.revision);
    assert_eq!(
        serde_json::to_value(&result).unwrap(),
        serde_json::to_value(cerebri_core::plan(source.clone())).unwrap()
    );
    assert_eq!(result.outcome, PlanningOutcome::Solution);
    assert_eq!(result.assessment, SearchAssessment::ProvenOptimal);
    assert_eq!(
        result
            .candidates
            .iter()
            .map(|c| serde_json::to_value(c.start)
                .unwrap()
                .as_str()
                .unwrap()
                .to_owned())
            .collect::<Vec<_>>(),
        [
            "2026-10-01T10:00:00Z",
            "2026-10-01T10:15:00Z",
            "2026-10-01T10:30:00Z",
            "2026-10-01T10:45:00Z",
            "2026-10-01T11:00:00Z",
            "2026-10-01T11:15:00Z",
            "2026-10-01T11:30:00Z",
        ]
    );
    for candidate in result.candidates {
        assert!(matches!(
            candidate
                .proposed
                .validate(&source.context)
                .unwrap()
                .into_action_plan(),
            Err(PlanError::AnalysisOnly)
        ));
    }
}

#[test]
fn rejects_each_profile_violation_without_silently_repairing_the_request() {
    use SuggestionRejection::*;
    for (path, replacement, expected) in [
        (
            "/integration_version/minor",
            json!(2),
            UnsupportedIntegrationVersion,
        ),
        (
            "/request/schema_version/minor",
            json!(1),
            UnsupportedCpirVersion,
        ),
        ("/request/operation", json!("CREATE"), UnsupportedOperation),
        (
            "/request/scope/max_mutations",
            json!(1),
            MutationAuthorityForbidden,
        ),
        (
            "/request/policy/snapshot/mutation/max_mutations",
            json!(1),
            MutationAuthorityForbidden,
        ),
        (
            "/request/policy/snapshot/mutation/allowed_actions",
            json!(["CREATE_EVENT"]),
            MutationAuthorityForbidden,
        ),
        (
            "/request/planning_capability/mutations",
            json!([{"object_id":"new-event","calendar_id":"synthetic-calendar","integration_id":"mock","kind":"CREATE_EVENT"}]),
            MutationAuthorityForbidden,
        ),
        (
            "/request/policy/snapshot/mode",
            json!("Confirmation"),
            UnsupportedDeploymentMode,
        ),
        ("/request/target_ids", json!([]), ProspectiveEventRequired),
        (
            "/request/context/objects/0/revision",
            json!(1),
            ProspectiveEventRequired,
        ),
        (
            "/request/context/objects/0/kind",
            json!({"kind":"RESOURCE"}),
            ProspectiveEventRequired,
        ),
        (
            "/request/context/temporal",
            Value::Null,
            TemporalCoverageRequired,
        ),
        (
            "/request/policy/snapshot/allow_uncertain_duration",
            json!(true),
            UncertainDurationForbidden,
        ),
    ] {
        let mut input = value();
        *input.pointer_mut(path).unwrap() = replacement;
        assert!(
            matches!(run(&input).outcome, SuggestionOutcome::Rejected { code } if code == expected),
            "{path}"
        );
    }
}

#[test]
fn core_still_blocks_unknown_coverage_permissions_duration_scope_and_bad_horizons() {
    for (path, replacement, issue) in [
        (
            "/request/context/temporal/coverage",
            json!("Incomplete"),
            "IncompleteCoverage",
        ),
        (
            "/request/planning_capability/read",
            json!(false),
            "PlanningPermissionDenied",
        ),
        (
            "/request/planning_capability/plan",
            json!(false),
            "PlanningPermissionDenied",
        ),
        (
            "/request/duration/value/knowledge",
            json!({"state":"UNKNOWN"}),
            "RequiredDuration",
        ),
        ("/request/scope/calendar_ids", json!([]), "ScopeViolation"),
        (
            "/request/context/temporal/horizon/end",
            json!("2026-10-01T13:00:00Z"),
            "HorizonMismatch",
        ),
        ("/request/budget/max_candidates", json!(4097), "InputLimit"),
    ] {
        let mut input = value();
        *input.pointer_mut(path).unwrap() = replacement;
        let SuggestionOutcome::Planned { result, .. } = run(&input).outcome else {
            panic!("{path}");
        };
        assert_eq!(
            result.outcome,
            PlanningOutcome::InsufficientInformation,
            "{path}"
        );
        assert!(result.candidates.is_empty());
        assert!(
            serde_json::to_string(&result.validation.issues)
                .unwrap()
                .contains(issue)
        );
        if issue == "IncompleteCoverage" {
            let availability = &result.compilation.unwrap().availability;
            assert!(availability.free.is_empty());
        }
    }
}

#[test]
fn decoding_is_closed_bounded_and_does_not_echo_input() {
    for bytes in [b"not-json-private-content".as_slice(), b"{}", &[0xff]] {
        let response = suggest_from_slice(bytes);
        assert!(matches!(
            response.outcome,
            SuggestionOutcome::Rejected {
                code: SuggestionRejection::InvalidRequest
            }
        ));
        assert!(
            !serde_json::to_string(&response)
                .unwrap()
                .contains("private-content")
        );
    }
    let mut input = value();
    input["private-content"] = json!(true);
    assert!(matches!(
        run(&input).outcome,
        SuggestionOutcome::Rejected {
            code: SuggestionRejection::InvalidRequest
        }
    ));
    assert!(matches!(
        suggest_from_slice(&vec![b' '; MAX_INTEGRATION_REQUEST_BYTES + 1]).outcome,
        SuggestionOutcome::Rejected {
            code: SuggestionRejection::RequestTooLarge
        }
    ));
}
