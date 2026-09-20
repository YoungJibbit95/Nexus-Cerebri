//! Development-only transport; execution authorization is intentionally not exposed.
use axum::{
    Json, Router,
    extract::DefaultBodyLimit,
    response::Redirect,
    routing::{get, post},
};
use cerebri_core::{
    PlanningRequest, PlanningResult, TemporalRequest, TemporalResult, ValidationReport,
};
use tower_http::services::ServeDir;

pub fn router() -> Router {
    Router::new()
        .route("/health",get(|| async { Json(serde_json::json!({
            "status":"ok","software_version":env!("CARGO_PKG_VERSION"),"release_state":"research","cpir_schema":"0.2", "supported_cpir_schemas":["0.1","0.2"]
        })) }))
        .route("/v1/validate",post(validate))
        .route("/v1/plan",post(plan))
        .route("/v1/temporal",post(temporal))
        .route("/lab",get(|| async { Redirect::permanent("/lab/") }))
        .nest_service("/lab/", ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../cerebri-lab/dist")))
        .layer(DefaultBodyLimit::max(256 * 1024))
}
async fn temporal(Json(request): Json<TemporalRequest>) -> Json<TemporalResult> {
    Json(cerebri_core::inspect_temporal(request))
}
async fn validate(Json(request): Json<PlanningRequest>) -> Json<ValidationReport> {
    Json(cerebri_core::validate(&request))
}
async fn plan(Json(request): Json<PlanningRequest>) -> Json<PlanningResult> {
    Json(cerebri_core::plan(request))
}
#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::{Body, to_bytes},
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;
    #[tokio::test]
    async fn temporal_route_preserves_typed_core_output_and_rejection() {
        let fixture = include_str!("../../../examples/temporal-request.json");
        let mut rejected: serde_json::Value = serde_json::from_str(fixture).unwrap();
        rejected["limits"]["max_occurrences"] = 0.into();
        for input in [fixture.to_string(), rejected.to_string()] {
            let response = router()
                .oneshot(
                    Request::post("/v1/temporal")
                        .header("content-type", "application/json")
                        .body(Body::from(input.clone()))
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            let actual: serde_json::Value =
                serde_json::from_slice(&to_bytes(response.into_body(), 1024 * 1024).await.unwrap())
                    .unwrap();
            let expected = serde_json::to_value(cerebri_core::inspect_temporal(
                serde_json::from_str(&input).unwrap(),
            ))
            .unwrap();
            assert_eq!(actual, expected);
        }
        let redirect = router()
            .oneshot(Request::get("/lab").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(redirect.status(), StatusCode::PERMANENT_REDIRECT);
        assert_eq!(redirect.headers()["location"], "/lab/");
    }
    #[tokio::test]
    async fn routes_delegate_to_core_and_expose_no_execution() {
        let input = include_str!("../../../examples/request.json");
        for path in ["/v1/plan", "/v1/validate"] {
            let response = router()
                .oneshot(
                    Request::post(path)
                        .header("content-type", "application/json")
                        .body(Body::from(input))
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            let actual: serde_json::Value =
                serde_json::from_slice(&to_bytes(response.into_body(), 1024 * 1024).await.unwrap())
                    .unwrap();
            let expected = if path.ends_with("plan") {
                cerebri_core::plan_json(input)
            } else {
                cerebri_core::validate_json(input)
            }
            .unwrap();
            assert_eq!(
                actual,
                serde_json::from_str::<serde_json::Value>(&expected).unwrap()
            );
        }
        let response = router()
            .oneshot(Request::get("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let response = router()
            .oneshot(Request::post("/v1/execute").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
    #[tokio::test]
    async fn malformed_and_oversized_bodies_are_rejected() {
        for (body, status) in [
            ("{}".to_string(), StatusCode::UNPROCESSABLE_ENTITY),
            (" ".repeat(256 * 1024 + 1), StatusCode::PAYLOAD_TOO_LARGE),
        ] {
            let response = router()
                .oneshot(
                    Request::post("/v1/plan")
                        .header("content-type", "application/json")
                        .body(Body::from(body))
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(response.status(), status);
        }
    }
}
