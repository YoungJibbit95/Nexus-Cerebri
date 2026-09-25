use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use tower::ServiceExt;

#[tokio::test]
async fn independent_scenarios_cross_api_core_and_planner_with_exact_parity() {
    let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/planner");
    let manifest: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(directory.join("manifest.json")).unwrap())
            .unwrap();
    for case in manifest["scenarios"].as_array().unwrap() {
        let input =
            std::fs::read_to_string(directory.join(case["file"].as_str().unwrap())).unwrap();
        let route = case["route"].as_str().unwrap();
        let response = cerebri_api::router()
            .oneshot(
                Request::post(route)
                    .header("content-type", "application/json")
                    .body(Body::from(input.clone()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK, "{}", case["name"]);
        let actual: serde_json::Value = serde_json::from_slice(
            &to_bytes(response.into_body(), 16 * 1024 * 1024)
                .await
                .unwrap(),
        )
        .unwrap();
        let expected = &case["expected"];
        if route == "/v1/temporal" {
            let core = cerebri_core::inspect_temporal(serde_json::from_str(&input).unwrap());
            assert_eq!(actual, serde_json::to_value(core).unwrap());
            assert_eq!(actual["status"], expected["status"]);
            assert_eq!(
                actual["data"]["expansions"][0]["skipped"]
                    .as_array()
                    .unwrap()
                    .len() as u64,
                expected["skipped"].as_u64().unwrap()
            );
            continue;
        }
        let request: cerebri_core::PlanningRequest = serde_json::from_str(&input).unwrap();
        let core = cerebri_core::plan(request.clone());
        assert_eq!(
            actual,
            serde_json::to_value(&core).unwrap(),
            "{}",
            case["name"]
        );
        for field in ["outcome", "assessment"] {
            assert_eq!(actual[field], expected[field], "{} {field}", case["name"]);
        }
        assert_eq!(
            core.candidates.len() as u64,
            expected["candidates"].as_u64().unwrap()
        );
        if let Some(start) = expected.get("first_start") {
            assert_eq!(actual["candidates"][0]["start"], *start);
        }
        if let Some(count) = expected.get("occurrences") {
            assert_eq!(
                actual["compilation"]["occurrences"]
                    .as_array()
                    .unwrap()
                    .len() as u64,
                count.as_u64().unwrap()
            );
        }
        if let Some(order) = expected.get("dependency_order") {
            assert_eq!(actual["dependency_graph"]["order"], *order);
        }
        if let Some(count) = expected.get("graph_issues") {
            assert_eq!(
                actual["dependency_graph"]["issues"]
                    .as_array()
                    .unwrap()
                    .len() as u64,
                count.as_u64().unwrap()
            );
        }
        if let Some(compilation) = expected.get("compilation") {
            assert_eq!(actual["compilation"], *compilation);
        }
        if let Some(count) = expected.get("unknown_ranges") {
            assert_eq!(
                actual["compilation"]["availability"]["unknown"]
                    .as_array()
                    .unwrap()
                    .len() as u64,
                count.as_u64().unwrap()
            );
        }
        if let Some(count) = expected.get("skipped") {
            assert_eq!(
                actual["compilation"]["series"][0]["expansion"]["skipped"]
                    .as_array()
                    .unwrap()
                    .len() as u64,
                count.as_u64().unwrap()
            );
        }
        // Feasibility and lifecycle proof are checked by the actual core, never fabricated in a fixture.
        for (wire, candidate) in actual["candidates"].as_array().unwrap().iter().zip(&core.candidates) {
            assert_eq!(wire["ranking_features"], serde_json::to_value(&candidate.ranking_features).unwrap());
            assert!(wire["ranking_features"].get("preferred_start_distance_seconds").is_some());
            assert!(wire["ranking_features"].get("preferred_start_source").is_some());
            assert_eq!(wire["ranking_features"]["schema_version"], serde_json::json!({"major":0,"minor":1}));
        }
        for candidate in core.candidates {
            candidate.proposed.validate(&request.context).unwrap();
        }
        let response = cerebri_api::router()
            .oneshot(
                Request::post("/v1/validate")
                    .header("content-type", "application/json")
                    .body(Body::from(input))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let validation: serde_json::Value =
            serde_json::from_slice(&to_bytes(response.into_body(), 1024 * 1024).await.unwrap())
                .unwrap();
        assert_eq!(
            validation,
            serde_json::to_value(cerebri_core::validate(&request)).unwrap()
        );
    }
}
