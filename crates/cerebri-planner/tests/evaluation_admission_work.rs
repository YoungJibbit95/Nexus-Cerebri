#[path = "../../../tests/support/mod.rs"]
mod support;
use cerebri_constraints::{ConstraintSpec, HardConstraint};
use cerebri_planner::evaluation::*;
use cerebri_planner::{ValidationIssue, validate_request};
use cerebri_types::Provenance;
use serde_json::{Value, json};

fn measured(bytes: u64, weighted: u64) -> Value {
    json!({"metric_version":"RUST_SERDE_JSON_1_0_151_PLANNING_REQUEST_V0_1",
        "serialized_request_bytes":{"state":"MEASURED","value":bytes.to_string()},
        "serialized_request_limit_bytes":"262144",
        "candidate_weighted_bytes":{"state":"SOME","value":weighted.to_string()},
        "candidate_weighted_limit_bytes":"16777216"})
}
#[test]
fn imported_measurements_require_the_exact_reachable_domain() {
    for (bytes, weighted, valid, admitted) in [
        (1, 1, true, true),
        (0, 0, false, false),
        (262144, 16777216, true, true),
        (262145, 262145, false, false),
        (1, 16777217, true, false),
        (1, 0, true, true),
        (2, 3, false, false),
    ] {
        let result = serde_json::from_value::<PlannerAdmissionWorkObservationV0_1>(measured(
            bytes, weighted,
        ));
        assert_eq!(result.is_ok(), valid);
        if let Ok(observation) = result {
            assert_eq!(observation.admitted(), admitted);
        }
    }
    let valid = measured(1024, 262144);
    let observed: PlannerAdmissionWorkObservationV0_1 =
        serde_json::from_value(valid.clone()).unwrap();
    observed.validate_budget(256).unwrap();
    assert!(observed.validate_budget(255).is_err());
    for field in valid.as_object().unwrap().keys() {
        let mut value = valid.clone();
        value.as_object_mut().unwrap().remove(field);
        assert!(serde_json::from_value::<PlannerAdmissionWorkObservationV0_1>(value).is_err());
        let mut value = valid.clone();
        value[field] = Value::Null;
        assert!(serde_json::from_value::<PlannerAdmissionWorkObservationV0_1>(value).is_err());
    }
    for (field, value) in [
        ("metric_version", json!("OTHER")),
        ("serialized_request_limit_bytes", json!("262145")),
        ("candidate_weighted_limit_bytes", json!("16777217")),
        ("extra", json!(true)),
    ] {
        let mut changed = valid.clone();
        changed[field] = value;
        assert!(serde_json::from_value::<PlannerAdmissionWorkObservationV0_1>(changed).is_err());
    }
    for state in ["EXCEEDS_SERIALIZATION_LIMIT", "SERIALIZATION_ERROR"] {
        let mut value = valid.clone();
        value["serialized_request_bytes"] = json!({"state":state});
        assert!(
            serde_json::from_value::<PlannerAdmissionWorkObservationV0_1>(value.clone()).is_err()
        );
        value["candidate_weighted_bytes"] = json!({"state":"NONE"});
        let result: PlannerAdmissionWorkObservationV0_1 = serde_json::from_value(value).unwrap();
        assert!(!result.admitted());
    }
}
#[test]
fn typed_request_observation_matches_independent_complete_serialization_and_existing_gate() {
    for budget in [0, 1, 64, 256, 4096, u32::MAX] {
        for padding in [0, 1, 1024, 262144] {
            let mut request = support::request();
            request.budget.max_candidates = budget;
            request.constraints.push(ConstraintSpec {
                object_id: request.target_ids[0].clone(),
                evidence: vec![],
                rule: HardConstraint::ExternalLock {
                    provenance: Provenance::SystemFact,
                    reason: "x".repeat(padding),
                },
            });
            let bytes = serde_json::to_vec(&request).unwrap().len() as u64;
            let expected = bytes <= 262144 && bytes * u64::from(budget) <= 16777216;
            let observation = PlannerAdmissionWorkObservationV0_1::measure(&request);
            assert_eq!(observation.admitted(), expected);
            if bytes <= 262144 {
                assert_eq!(
                    observation.wire().serialized_request_bytes,
                    SerializedRequestMeasurementV0_1::Measured {
                        value: CanonicalU64Decimal::new(bytes).unwrap()
                    }
                );
                observation.validate_budget(budget).unwrap();
            } else {
                assert_eq!(
                    observation.wire().serialized_request_bytes,
                    SerializedRequestMeasurementV0_1::ExceedsSerializationLimit {}
                );
                assert_eq!(
                    observation.wire().candidate_weighted_bytes,
                    CanonicalOptionalV1::None {}
                );
            }
            if !expected {
                assert!(
                    validate_request(&request)
                        .issues
                        .contains(&ValidationIssue::InputLimit)
                );
            }
        }
    }
}
#[test]
fn excluded_prose_still_changes_the_typed_request_work_metric() {
    let mut request = support::request();
    request.constraints.push(ConstraintSpec {
        object_id: request.target_ids[0].clone(),
        evidence: vec![],
        rule: HardConstraint::ExternalLock {
            provenance: Provenance::SystemFact,
            reason: String::new(),
        },
    });
    let before = PlannerAdmissionWorkObservationV0_1::measure(&request);
    if let HardConstraint::ExternalLock { reason, .. } =
        &mut request.constraints.last_mut().unwrap().rule
    {
        reason.push('x');
    }
    let after = PlannerAdmissionWorkObservationV0_1::measure(&request);
    let length = |o: &PlannerAdmissionWorkObservationV0_1| match o.wire().serialized_request_bytes {
        SerializedRequestMeasurementV0_1::Measured { value } => value.value(),
        _ => panic!("expected measured"),
    };
    assert_eq!(length(&after), length(&before) + 1);
}
