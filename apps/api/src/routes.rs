//! The HTTP surface. Each endpoint lives in its own module and registers itself here.

mod caching;
mod content;
mod health;
mod llms_full_txt;
mod llms_txt;
mod response;
mod robots_txt;
mod sitemap_xml;

use axum::routing::get;
use axum::{
    Router,
    middleware,
};

use crate::state::AppState;

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::check_health))
        .route("/llms.txt", get(llms_txt::serve_llms_txt))
        .route("/llms-full.txt", get(llms_full_txt::serve_llms_full_txt))
        .route("/sitemap.xml", get(sitemap_xml::serve_sitemap_xml))
        .route("/robots.txt", get(robots_txt::serve_robots_txt))
        .route("/", get(content::serve_root))
        .route("/{*path}", get(content::serve_path))
        .layer(middleware::from_fn(caching::add_caching))
        .with_state(state)
}
