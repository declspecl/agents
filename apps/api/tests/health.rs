use api::routes::build_router;
use axum::body::Body;
use axum::http::{
    Request,
    StatusCode,
};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn health_reports_ok() {
    let response = build_router().oneshot(Request::get("/health").body(Body::empty()).unwrap()).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert_eq!(&body[..], br#"{"status":"ok"}"#);
}

#[tokio::test]
async fn unknown_paths_are_not_found() {
    let response = build_router().oneshot(Request::get("/nope").body(Body::empty()).unwrap()).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
