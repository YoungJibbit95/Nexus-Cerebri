use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use tower::ServiceExt;

#[tokio::test]
async fn explicit_runtime_assets_preserve_routes_and_development_default() {
    let path = std::env::temp_dir().join(format!(
        "cerebri runtime assets {} {}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(path.join("assets")).unwrap();
    std::fs::write(path.join("index.html"), "<html>runtime Lab</html>").unwrap();
    std::fs::write(path.join("assets/test.js"), "runtime_asset").unwrap();
    for (url, body) in [
        ("/lab/", "<html>runtime Lab</html>"),
        ("/lab/assets/test.js", "runtime_asset"),
    ] {
        let response = cerebri_api::router_with_lab_dir(&path)
            .oneshot(Request::get(url).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            to_bytes(response.into_body(), 1024).await.unwrap().as_ref(),
            body.as_bytes()
        );
    }
    for route in [
        "/health",
        "/lab/",
        "/v1/plan",
        "/v1/validate",
        "/v1/temporal",
        "/v1/execute",
    ] {
        let input = match route {
            "/v1/plan" | "/v1/validate" => include_str!("../../../examples/request.json"),
            "/v1/temporal" => include_str!("../../../examples/temporal-request.json"),
            _ => "",
        };
        let request = || {
            Request::builder()
                .method(if route.starts_with("/v1/") {
                    "POST"
                } else {
                    "GET"
                })
                .uri(route)
                .header("content-type", "application/json")
                .body(Body::from(input))
                .unwrap()
        };
        let default = cerebri_api::router().oneshot(request()).await.unwrap();
        let supplied = cerebri_api::router_with_lab_dir(&path)
            .oneshot(request())
            .await
            .unwrap();
        if route == "/lab/" {
            let built = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../cerebri-lab/dist/index.html");
            assert_eq!(
                default.status(),
                if built.exists() {
                    StatusCode::OK
                } else {
                    StatusCode::NOT_FOUND
                }
            );
            if built.exists() {
                assert_eq!(
                    to_bytes(default.into_body(), 1024 * 1024)
                        .await
                        .unwrap()
                        .as_ref(),
                    std::fs::read(built).unwrap()
                );
            }
        } else {
            assert_eq!(default.status(), supplied.status());
            if route == "/v1/execute" {
                assert_eq!(supplied.status(), StatusCode::NOT_FOUND);
            }
            assert_eq!(
                to_bytes(default.into_body(), 4 * 1024 * 1024)
                    .await
                    .unwrap(),
                to_bytes(supplied.into_body(), 4 * 1024 * 1024)
                    .await
                    .unwrap()
            );
        }
    }
    std::fs::remove_dir_all(path).unwrap();
}

#[test]
fn binary_rejects_non_loopback_before_listening() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_cerebri-api"))
        .env("CEREBRI_BIND_ADDR", "0.0.0.0:0")
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("requires loopback"));
    assert!(!String::from_utf8_lossy(&output.stdout).contains("CEREBRI_LISTEN_ADDR="));
}
