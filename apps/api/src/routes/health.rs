use axum::Json;
use axum::http::header;
use axum::response::IntoResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct Health {
    status: &'static str,
}

/// Never cached: a health check that answers from a cache says nothing about the service.
pub async fn check_health() -> impl IntoResponse {
    (
        [(header::CACHE_CONTROL, "no-store")],
        Json(Health {
            status: "ok",
        }),
    )
}
