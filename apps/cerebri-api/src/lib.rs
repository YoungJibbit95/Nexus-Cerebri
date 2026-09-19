//! Development-only transport; execution authorization is intentionally not exposed.
use axum::{
    Json, Router,
    extract::DefaultBodyLimit,
    response::Html,
    routing::{get, post},
};
use cerebri_core::{PlanningRequest, PlanningResult, ValidationReport};

pub fn router() -> Router {
    Router::new()
        .route("/health",get(|| async { Json(serde_json::json!({
            "status":"ok","software_version":env!("CARGO_PKG_VERSION"),"release_state":"unreleased","cpir_schema":"0.1"
        })) }))
        .route("/v1/validate",post(validate))
        .route("/v1/plan",post(plan))
        .route("/lab",get(|| async { Html(include_str!("../../cerebri-lab/index.html")) }))
        .layer(DefaultBodyLimit::max(256 * 1024))
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
