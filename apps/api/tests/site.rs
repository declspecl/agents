#![allow(clippy::unwrap_used, reason = "tests and their helpers may unwrap")]

mod support;

use axum::http::StatusCode;
use support::{
    fixture_catalog,
    get,
    load,
};

#[tokio::test]
async fn llms_txt_indexes_every_document_by_section() {
    let reply = get(fixture_catalog(), "/llms.txt").await;

    assert_eq!(reply.status, StatusCode::OK);
    assert_eq!(reply.content_type, "text/plain; charset=utf-8");
    assert_eq!(
        reply.body,
        "\
# Gavin Dhondt's agent setup

> Gavin's setup, tools and working preferences, written for AI agents. Read this to learn how to work with him.

Every link below is a plain markdown document. A path ending in `/` lists its contents. Every document is listed in https://agents.example.com/sitemap.xml, and all of them in one file at https://agents.example.com/llms-full.txt.

## Overview

- [About](https://agents.example.com/about.md): What this is.

## Preferences

- [Editor](https://agents.example.com/preferences/editor.md): Editor choices.
- [Shell](https://agents.example.com/preferences/shell.md): Shell choices.

## Setup

- [Tools](https://agents.example.com/setup/macos/tools.md): Installed tools.
"
    );
}

#[tokio::test]
async fn llms_full_txt_concatenates_every_document_with_its_source() {
    let reply = get(fixture_catalog(), "/llms-full.txt").await;

    assert_eq!(reply.status, StatusCode::OK);
    assert_eq!(reply.content_type, "text/plain; charset=utf-8");
    assert!(reply.body.starts_with("# Gavin Dhondt's agent setup\n"));
    for expected in [
        "Source: https://agents.example.com/about.md\n\n# About\n\nBody of About.",
        "Source: https://agents.example.com/preferences/shell.md\n\n# Shell\n\nBody of Shell.",
        "Source: https://agents.example.com/setup/macos/tools.md\n\n# Tools\n\nBody of Tools.",
    ] {
        assert!(reply.body.contains(expected), "missing {expected:?} in:\n{}", reply.body);
    }
}

#[tokio::test]
async fn sitemap_lists_every_directory_and_document_with_absolute_urls() {
    let reply = get(fixture_catalog(), "/sitemap.xml").await;

    assert_eq!(reply.status, StatusCode::OK);
    assert_eq!(reply.content_type, "application/xml; charset=utf-8");
    assert_eq!(
        reply.body,
        "\
<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">
  <url><loc>https://agents.example.com/</loc></url>
  <url><loc>https://agents.example.com/preferences/</loc></url>
  <url><loc>https://agents.example.com/setup/</loc></url>
  <url><loc>https://agents.example.com/setup/macos/</loc></url>
  <url><loc>https://agents.example.com/about.md</loc></url>
  <url><loc>https://agents.example.com/preferences/editor.md</loc></url>
  <url><loc>https://agents.example.com/preferences/shell.md</loc></url>
  <url><loc>https://agents.example.com/setup/macos/tools.md</loc></url>
</urlset>
"
    );
}

#[tokio::test]
async fn robots_txt_allows_everything_and_points_at_the_maps() {
    let reply = get(fixture_catalog(), "/robots.txt").await;

    assert_eq!(reply.status, StatusCode::OK);
    assert_eq!(reply.content_type, "text/plain; charset=utf-8");
    assert!(reply.body.starts_with("User-agent: *\nAllow: /\n"));
    assert!(reply.body.contains("Sitemap: https://agents.example.com/sitemap.xml"));
    assert!(reply.body.contains("https://agents.example.com/llms.txt"));
}

#[tokio::test]
async fn serves_a_document_as_markdown_without_its_front_matter() {
    let reply = get(fixture_catalog(), "/preferences/editor.md").await;

    assert_eq!(reply.status, StatusCode::OK);
    assert_eq!(reply.content_type, "text/plain; charset=utf-8");
    assert_eq!(reply.body, "# Editor\n\nBody of Editor.\n");
}

#[tokio::test]
async fn lists_a_directory_with_a_link_to_its_parent() {
    let reply = get(fixture_catalog(), "/preferences/").await;

    assert_eq!(reply.status, StatusCode::OK);
    assert_eq!(reply.content_type, "text/plain; charset=utf-8");
    assert_eq!(
        reply.body,
        "\
# Index of /preferences/

- [../](/)
- [Editor](/preferences/editor.md): Editor choices.
- [Shell](/preferences/shell.md): Shell choices.
"
    );
}

#[tokio::test]
async fn a_nested_directory_links_to_its_own_parent() {
    let reply = get(fixture_catalog(), "/setup/macos/").await;

    assert!(reply.body.contains("- [../](/setup/)"), "{}", reply.body);
    assert!(reply.body.contains("- [Tools](/setup/macos/tools.md): Installed tools."));
}

#[tokio::test]
async fn lists_the_root_with_its_directories_and_documents() {
    let reply = get(fixture_catalog(), "/").await;

    assert_eq!(reply.status, StatusCode::OK);
    assert_eq!(
        reply.body,
        "\
# Index of /

- [/preferences/](/preferences/)
- [/setup/](/setup/)
- [About](/about.md): What this is.
"
    );
}

#[tokio::test]
async fn the_root_lists_cleanly_when_there_is_no_content() {
    let reply = get(load(&[]), "/").await;

    assert_eq!(reply.status, StatusCode::OK);
    assert_eq!(reply.body, "# Index of /\n");
}

#[tokio::test]
async fn redirects_a_directory_without_its_slash() {
    let reply = get(fixture_catalog(), "/preferences").await;

    assert_eq!(reply.status, StatusCode::PERMANENT_REDIRECT);
    assert_eq!(reply.location.as_deref(), Some("/preferences/"));
}

#[tokio::test]
async fn unknown_paths_are_not_found_and_point_back_to_the_index() {
    for path in ["/nope", "/nope/", "/nope.md", "/preferences/missing.md", "/preferences/editor", "/Preferences/", "/preferences/../about.md", "/pref"] {
        let reply = get(fixture_catalog(), path).await;

        assert_eq!(reply.status, StatusCode::NOT_FOUND, "{path}");
        assert_eq!(reply.content_type, "text/plain; charset=utf-8", "{path}");
        assert!(reply.body.contains("/llms.txt"), "{path}");
    }
}

#[tokio::test]
async fn the_not_found_body_names_the_missing_path() {
    let reply = get(fixture_catalog(), "/preferences/missing.md").await;

    assert!(reply.body.contains("`/preferences/missing.md`"));
}

#[tokio::test]
async fn only_reads_are_allowed() {
    use api::routes::build_router;
    use api::state::AppState;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;

    let router = build_router(AppState::new(fixture_catalog(), support::base_url()));
    let response = router.oneshot(Request::post("/llms.txt").body(Body::empty()).unwrap()).await.unwrap();

    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn head_requests_are_answered_like_gets_without_a_body() {
    use api::routes::build_router;
    use api::state::AppState;
    use axum::body::Body;
    use axum::http::Request;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    let router = build_router(AppState::new(fixture_catalog(), support::base_url()));
    let response = router.oneshot(Request::head("/about.md").body(Body::empty()).unwrap()).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert!(response.into_body().collect().await.unwrap().to_bytes().is_empty());
}

#[tokio::test]
async fn health_reports_ok() {
    let reply = get(fixture_catalog(), "/health").await;

    assert_eq!(reply.status, StatusCode::OK);
    assert_eq!(reply.body, r#"{"status":"ok"}"#);
}
