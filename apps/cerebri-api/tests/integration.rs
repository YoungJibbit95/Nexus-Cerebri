use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use cerebri_core::integration;
use serde_json::{Value, json};
use tower::ServiceExt;

const FIXTURE: &str = include_str!("../../../examples/integration/suggestion.json");

async fn post(input: &str) -> (StatusCode, Value) {
    let response = cerebri_api::router()
        .oneshot(
            Request::post("/v1/integration/suggestions")
                .header("content-type", "application/json")
                .body(Body::from(input.to_owned()))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    (
        status,
        serde_json::from_slice(
            &to_bytes(response.into_body(), 16 * 1024 * 1024)
                .await
                .unwrap(),
        )
        .unwrap(),
    )
}

#[tokio::test]
async fn manifest_and_suggestions_are_exact_core_contracts() {
    let response = cerebri_api::router()
        .oneshot(
            Request::get("/v1/integration/manifest")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let manifest: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 4096).await.unwrap()).unwrap();
    assert_eq!(
        manifest,
        serde_json::to_value(integration::describe()).unwrap()
    );
    let mut incomplete: Value = serde_json::from_str(FIXTURE).unwrap();
    incomplete["request"]["context"]["temporal"]["coverage"] = json!("Incomplete");
    for input in [FIXTURE.to_owned(), incomplete.to_string()] {
        let (status, actual) = post(&input).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            actual,
            serde_json::to_value(integration::suggest_from_slice(input.as_bytes())).unwrap()
        );
    }
}

#[tokio::test]
async fn rejections_are_typed_sanitized_and_bounded() {
    let mut mutation: Value = serde_json::from_str(FIXTURE).unwrap();
    mutation["request"]["scope"]["max_mutations"] = json!(1);
    for (input, status, code) in [
        (
            mutation.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
            "mutation_authority_forbidden",
        ),
        (
            "{}".to_owned(),
            StatusCode::UNPROCESSABLE_ENTITY,
            "invalid_request",
        ),
        (
            "private-invalid-json".to_owned(),
            StatusCode::BAD_REQUEST,
            "invalid_request",
        ),
        (
            " ".repeat(256 * 1024 + 1),
            StatusCode::PAYLOAD_TOO_LARGE,
            "request_too_large",
        ),
    ] {
        let (actual_status, actual) = post(&input).await;
        assert_eq!(actual_status, status);
        assert_eq!(
            actual,
            json!({"integration_version":{"major":0,"minor":1},"status":"rejected","code":code})
        );
    }
    let response = cerebri_api::router()
        .oneshot(
            Request::post("/v1/integration/execute")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
