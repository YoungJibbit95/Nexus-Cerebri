#[path = "../../../tests/support/corpus.rs"]
mod corpus;
#[path = "../../../tests/support/generator.rs"]
mod generator;
use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use cerebri_core::PlanningRequest;
use tower::ServiceExt;

async fn post(route: &str, body: String) -> (StatusCode, Vec<u8>) {
    let response = cerebri_api::router()
        .oneshot(
            Request::post(route)
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    (
        response.status(),
        to_bytes(response.into_body(), 16 * 1024 * 1024)
            .await
            .unwrap()
            .to_vec(),
    )
}

#[tokio::test]
async fn malformed_corpus_fails_closed_without_panics_and_with_core_api_agreement() {
    for (name, body) in corpus::malformed_cases() {
        let parsed = std::panic::catch_unwind(|| serde_json::from_str::<PlanningRequest>(&body))
            .expect("parser must not panic");
        for route in ["/v1/validate", "/v1/plan"] {
            let (status, bytes) = post(route, body.clone()).await;
            match &parsed {
                Err(_) => assert!(status.is_client_error(), "{name} {route}: {status}"),
                Ok(request) => {
                    assert_eq!(status, StatusCode::OK, "{name}");
                    let result = std::panic::catch_unwind(|| cerebri_core::plan(request.clone()))
                        .expect("planner must not panic");
                    assert_eq!(
                        serde_json::to_value(result.outcome).unwrap(),
                        "InsufficientInformation",
                        "{name}"
                    );
                    assert_eq!(
                        serde_json::to_value(result.assessment).unwrap(),
                        "BestFound",
                        "{name}"
                    );
                    assert!(result.candidates.is_empty(), "{name}");
                    let validation = cerebri_core::validate(request);
                    assert_eq!(result.validation, validation, "{name}");
                    let expected = if route.ends_with("plan") {
                        serde_json::to_value(result).unwrap()
                    } else {
                        serde_json::to_value(validation).unwrap()
                    };
                    assert_eq!(
                        serde_json::from_slice::<serde_json::Value>(&bytes).unwrap(),
                        expected,
                        "{name}"
                    );
                }
            }
        }
    }
    for route in ["/v1/validate", "/v1/plan"] {
        assert_eq!(
            post(route, " ".repeat(256 * 1024 + 1)).await.0,
            StatusCode::PAYLOAD_TOO_LARGE
        );
    }
}

#[tokio::test]
async fn schema_roundtrip_and_api_compatibility_are_explicit() {
    let base: PlanningRequest = serde_json::from_str(include_str!(
        "../../../examples/planner/recurrence-busy.json"
    ))
    .unwrap();
    for minor in [0, 1, 2, 3, u16::MAX] {
        for temporal in [false, true] {
            let mut request = base.clone();
            request.schema_version.minor = minor;
            if !temporal {
                request.context.temporal = None;
            }
            let wire = serde_json::to_string(&request).unwrap();
            let roundtrip: PlanningRequest = serde_json::from_str(&wire).unwrap();
            assert_eq!(request, roundtrip);
            let accepted = minor == 2 || (minor == 1 && !temporal);
            let plan = cerebri_core::plan(roundtrip.clone());
            assert_eq!(
                serde_json::to_value(plan.outcome).unwrap() == "Solution",
                accepted,
                "minor={minor}, temporal={temporal}"
            );
            assert_eq!(plan.validation, cerebri_core::validate(&roundtrip));
            for route in ["/v1/validate", "/v1/plan"] {
                let (status, body) = post(route, wire.clone()).await;
                assert_eq!(status, StatusCode::OK);
                let expected = if route.ends_with("plan") {
                    serde_json::to_value(&plan).unwrap()
                } else {
                    serde_json::to_value(&plan.validation).unwrap()
                };
                assert_eq!(
                    serde_json::from_slice::<serde_json::Value>(&body).unwrap(),
                    expected
                );
            }
            if !temporal {
                assert!(plan.compilation.is_none());
            }
        }
    }
}

#[tokio::test]
async fn seeded_bounded_byte_mutations_do_not_panic_or_diverge() {
    let source = include_bytes!("../../../examples/request.json");
    for seed in 0..128 {
        let mut rng = generator::Generator(seed);
        let mut bytes = source.to_vec();
        let offset = rng.pick(bytes.len() as u64) as usize;
        match rng.pick(3) {
            0 => {
                bytes.truncate(offset);
            }
            1 => {
                bytes.remove(offset);
            }
            _ => bytes[offset] = b"{}[]\"0:x "[rng.pick(9) as usize],
        }
        let body = String::from_utf8(bytes).unwrap();
        for route in ["/v1/validate", "/v1/plan"] {
            let parsed = std::panic::catch_unwind(|| {
                if route.ends_with("plan") {
                    cerebri_core::plan_json(&body)
                } else {
                    cerebri_core::validate_json(&body)
                }
            })
            .unwrap_or_else(|_| panic!("panic seed={seed}, route={route}, body={body}"));
            let (status, response) = post(route, body.clone()).await;
            match parsed {
                Ok(expected) => {
                    assert_eq!(status, StatusCode::OK, "seed={seed}");
                    assert_eq!(
                        serde_json::from_slice::<serde_json::Value>(&response).unwrap(),
                        serde_json::from_str::<serde_json::Value>(&expected).unwrap(),
                        "seed={seed}"
                    );
                }
                Err(_) => assert!(status.is_client_error(), "seed={seed}"),
            }
        }
    }
}

#[tokio::test]
async fn extreme_local_date_is_a_rejection_instead_of_an_http_task_panic() {
    let mut value: serde_json::Value =
        serde_json::from_str(include_str!("../../../examples/request.json")).unwrap();
    value["context"]["objects"]
        .as_array_mut()
        .unwrap()
        .truncate(1);
    value["scope"]["time_range"] =
        serde_json::json!({"start":"+262142-12-31T23:59:57Z","end":"+262142-12-31T23:59:59Z"});
    value["duration"]["value"]["knowledge"]["data"] = 1.into();
    value["granularity"] = 1.into();
    value["constraints"] = serde_json::json!([{"object_id":"new-event","rule":{"kind":"EXPLICIT_DATE","value":"+262142-12-31"},"evidence":[]}]);
    let body = value.to_string();
    let core = cerebri_core::plan_json(&body).unwrap();
    let (status, actual) = post("/v1/plan", body).await;
    assert_eq!(status, StatusCode::OK);
    let result: serde_json::Value = serde_json::from_slice(&actual).unwrap();
    assert_eq!(
        result,
        serde_json::from_str::<serde_json::Value>(&core).unwrap()
    );
    assert_eq!(result["outcome"], "NoSolution");
    assert_eq!(result["assessment"], "Complete");
}
