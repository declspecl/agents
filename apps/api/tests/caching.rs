#![allow(clippy::unwrap_used, reason = "tests and their helpers may unwrap")]

mod support;

use axum::http::StatusCode;
use support::{
    fixture_catalog,
    get,
    get_with,
};

const CACHEABLE: &str = "public, max-age=300";

async fn etag_of(path: &str) -> String {
    get(fixture_catalog(), path).await.etag.unwrap()
}

#[tokio::test]
async fn successful_responses_are_cacheable_and_carry_a_strong_etag() {
    for path in ["/", "/about.md", "/preferences/", "/llms.txt", "/llms-full.txt", "/sitemap.xml", "/robots.txt"] {
        let reply = get(fixture_catalog(), path).await;

        assert_eq!(reply.status, StatusCode::OK, "{path}");
        assert_eq!(reply.cache_control.as_deref(), Some(CACHEABLE), "{path}");
        let etag = reply.etag.unwrap_or_else(|| panic!("{path} has no ETag"));
        assert!(etag.starts_with('"') && etag.ends_with('"') && !etag.starts_with("W/"), "{path}: {etag}");
    }
}

#[tokio::test]
async fn the_etag_is_stable_for_the_same_body_and_differs_between_bodies() {
    assert_eq!(etag_of("/about.md").await, etag_of("/about.md").await);
    assert_ne!(etag_of("/about.md").await, etag_of("/preferences/editor.md").await);
}

#[tokio::test]
async fn the_etag_follows_the_served_body_and_not_the_front_matter() {
    use support::{
        document,
        load,
    };

    let before = get(load(&[("about.md", document("About", "One."))]), "/about.md").await.etag.unwrap();
    let after = get(load(&[("about.md", document("About", "Two."))]), "/about.md").await;

    // The body is the same text under the same title, so the summary (front matter) does not change it.
    assert_eq!(after.etag.unwrap(), before);

    let changed = get(load(&[("about.md", document("About changed", "One."))]), "/about.md").await;
    assert_ne!(changed.etag.unwrap(), before);
}

#[tokio::test]
async fn a_matching_if_none_match_gets_a_bodyless_304() {
    let etag = etag_of("/about.md").await;
    let reply = get_with(fixture_catalog(), "/about.md", &[("if-none-match", &etag)]).await;

    assert_eq!(reply.status, StatusCode::NOT_MODIFIED);
    assert!(reply.body.is_empty());
    assert_eq!(reply.etag.as_deref(), Some(etag.as_str()));
    assert_eq!(reply.cache_control.as_deref(), Some(CACHEABLE));
}

#[tokio::test]
async fn a_stale_if_none_match_gets_the_full_response() {
    let reply = get_with(fixture_catalog(), "/about.md", &[("if-none-match", "\"0000000000000000\"")]).await;

    assert_eq!(reply.status, StatusCode::OK);
    assert!(reply.body.starts_with("# About"));
}

#[tokio::test]
async fn if_none_match_accepts_weak_tags_lists_and_a_wildcard() {
    let etag = etag_of("/about.md").await;

    for header in [format!("W/{etag}"), format!("\"stale\", {etag}"), format!("{etag} , \"other\""), "*".to_owned()] {
        let reply = get_with(fixture_catalog(), "/about.md", &[("if-none-match", &header)]).await;

        assert_eq!(reply.status, StatusCode::NOT_MODIFIED, "{header}");
    }
}

#[tokio::test]
async fn one_documents_etag_does_not_validate_another() {
    let other = etag_of("/preferences/editor.md").await;
    let reply = get_with(fixture_catalog(), "/about.md", &[("if-none-match", &other)]).await;

    assert_eq!(reply.status, StatusCode::OK);
}

#[tokio::test]
async fn failures_are_never_cached() {
    let reply = get(fixture_catalog(), "/nope").await;

    assert_eq!(reply.status, StatusCode::NOT_FOUND);
    assert_eq!(reply.cache_control.as_deref(), Some("no-store"));
    assert!(reply.etag.is_none());
}

#[tokio::test]
async fn a_not_found_is_not_turned_into_a_304() {
    let reply = get_with(fixture_catalog(), "/nope", &[("if-none-match", "*")]).await;

    assert_eq!(reply.status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn health_is_never_cached() {
    let reply = get(fixture_catalog(), "/health").await;

    assert_eq!(reply.status, StatusCode::OK);
    assert_eq!(reply.cache_control.as_deref(), Some("no-store"));
    assert!(reply.etag.is_none());
}

#[tokio::test]
async fn redirects_carry_no_etag() {
    let reply = get(fixture_catalog(), "/preferences").await;

    assert_eq!(reply.status, StatusCode::PERMANENT_REDIRECT);
    assert!(reply.etag.is_none());
}
