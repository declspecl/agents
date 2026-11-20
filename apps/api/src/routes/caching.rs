//! Cache headers and conditional requests for every response, applied in one place so no endpoint has to know about them.
//!
//! Content only changes when a new binary is deployed, so a successful response is safe to cache briefly and to
//! revalidate cheaply: its `ETag` is a hash of its body, and a request whose `If-None-Match` still matches gets a 304
//! with no body. Failures are never cached.

use std::hash::{
    DefaultHasher,
    Hash,
    Hasher,
};

use axum::body::{
    Body,
    to_bytes,
};
use axum::extract::Request;
use axum::http::{
    HeaderValue,
    StatusCode,
    header,
};
use axum::middleware::Next;
use axum::response::Response;

/// Five minutes: short enough that a deploy shows up soon, long enough to spare repeat fetches.
const CACHEABLE: &str = "public, max-age=300";
const UNCACHEABLE: &str = "no-store";
/// The largest body the layer will hash. Every response is far smaller; this only bounds a bug.
const MAX_BODY_BYTES: usize = 16 * 1024 * 1024;

/// Adds `Cache-Control` and `ETag`, and answers a matching conditional request with 304.
///
/// A response that already sets its own `Cache-Control` (such as `/health`) is left alone.
pub async fn add_caching(
    request: Request,
    next: Next,
) -> Response {
    let if_none_match = request.headers().get(header::IF_NONE_MATCH).cloned();
    let response = next.run(request).await;

    if response.headers().contains_key(header::CACHE_CONTROL) {
        return response;
    }

    let status = response.status();
    if status == StatusCode::OK {
        return cache_success(response, if_none_match.as_ref()).await;
    }
    if status == StatusCode::NOT_FOUND {
        return with_cache_control(response, UNCACHEABLE);
    }

    response
}

async fn cache_success(
    response: Response,
    if_none_match: Option<&HeaderValue>,
) -> Response {
    let (mut parts, body) = response.into_parts();
    let Ok(bytes) = to_bytes(body, MAX_BODY_BYTES).await else {
        return with_cache_control(Response::from_parts(parts, Body::empty()), UNCACHEABLE);
    };

    let etag = compute_etag(&bytes);
    let etag_value = HeaderValue::from_str(&etag);
    let cache_control = HeaderValue::from_static(CACHEABLE);

    if let (Ok(etag_value), Some(if_none_match)) = (&etag_value, if_none_match)
        && matches_etag(if_none_match, &etag)
    {
        let mut not_modified = Response::new(Body::empty());
        *not_modified.status_mut() = StatusCode::NOT_MODIFIED;
        not_modified.headers_mut().insert(header::ETAG, etag_value.clone());
        not_modified.headers_mut().insert(header::CACHE_CONTROL, cache_control);

        return not_modified;
    }

    if let Ok(etag_value) = etag_value {
        parts.headers.insert(header::ETAG, etag_value);
    }
    parts.headers.insert(header::CACHE_CONTROL, cache_control);

    Response::from_parts(parts, Body::from(bytes))
}

fn with_cache_control(
    mut response: Response,
    value: &'static str,
) -> Response {
    response.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static(value));

    response
}

/// A strong validator: the quoted hash of the body. The hasher is deterministic, so every instance running the same
/// binary agrees, and it changes whenever the body does.
fn compute_etag(body: &[u8]) -> String {
    let mut hasher = DefaultHasher::new();
    body.hash(&mut hasher);

    format!("\"{:016x}\"", hasher.finish())
}

/// Whether an `If-None-Match` header (`*`, or a comma-separated list of tags, weak or strong) names `etag`.
fn matches_etag(
    if_none_match: &HeaderValue,
    etag: &str,
) -> bool {
    let Ok(listed) = if_none_match.to_str() else {
        return false;
    };

    listed.split(',').map(str::trim).any(|candidate| candidate == "*" || candidate.strip_prefix("W/").unwrap_or(candidate) == etag)
}
