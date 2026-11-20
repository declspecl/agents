//! Shared fixtures: a small content tree, and helpers to build catalogs and call the router.
//!
//! Each test binary compiles this module on its own and uses only part of it.

#![allow(dead_code, clippy::unwrap_used, reason = "shared by several test binaries; tests and their helpers may unwrap")]

use api::content::{
    Catalog,
    Source,
};
use api::environment::BaseUrl;
use api::routes::build_router;
use api::state::AppState;
use axum::body::Body;
use axum::http::{
    Request,
    StatusCode,
};
use http_body_util::BodyExt;
use tower::ServiceExt;

/// A document with the given title and summary, in the format a source file has.
pub fn document(
    title: &str,
    summary: &str,
) -> String {
    format!("+++\ntitle = \"{title}\"\nsummary = \"{summary}\"\n+++\n\n# {title}\n\nBody of {title}.\n")
}

pub fn fixture_sources() -> Vec<(&'static str, String)> {
    vec![
        ("about.md", document("About", "What this is.")),
        ("preferences/editor.md", document("Editor", "Editor choices.")),
        ("preferences/shell.md", document("Shell", "Shell choices.")),
        ("setup/macos/tools.md", document("Tools", "Installed tools.")),
    ]
}

pub fn load(sources: &[(&str, String)]) -> Catalog {
    let sources = sources
        .iter()
        .map(|(path, text)| Source {
            path: (*path).to_owned(),
            text,
        })
        .collect();

    Catalog::load_from(sources, Vec::new()).unwrap()
}

pub fn fixture_catalog() -> Catalog {
    load(&fixture_sources())
}

pub fn base_url() -> BaseUrl {
    "https://agents.example.com".parse().unwrap()
}

pub struct Reply {
    pub status: StatusCode,
    pub content_type: String,
    pub location: Option<String>,
    pub cache_control: Option<String>,
    pub etag: Option<String>,
    pub body: String,
}

/// Sends a GET to a router over `catalog` and collects the whole response.
pub async fn get(
    catalog: Catalog,
    path: &str,
) -> Reply {
    get_with(catalog, path, &[]).await
}

/// Like [`get`], with extra request headers.
pub async fn get_with(
    catalog: Catalog,
    path: &str,
    headers: &[(&str, &str)],
) -> Reply {
    let router = build_router(AppState::new(catalog, base_url()));
    let mut request = Request::get(path);
    for (name, value) in headers {
        request = request.header(*name, *value);
    }
    let response = router.oneshot(request.body(Body::empty()).unwrap()).await.unwrap();

    let status = response.status();
    let header = |name: &str| response.headers().get(name).map(|value| value.to_str().unwrap().to_owned());
    let content_type = header("content-type").unwrap_or_default();
    let location = header("location");
    let cache_control = header("cache-control");
    let etag = header("etag");
    let body = response.into_body().collect().await.unwrap().to_bytes();

    Reply {
        status,
        content_type,
        location,
        cache_control,
        etag,
        body: String::from_utf8(body.to_vec()).unwrap(),
    }
}
