use askama::Template;

use super::{
    SITE_SUMMARY,
    SITE_TITLE,
};
use crate::content::{
    Catalog,
    Document,
};
use crate::environment::BaseUrl;

/// A document and where it came from.
pub struct FullDocument {
    pub url: String,
    pub body: String,
}

/// `/llms-full.txt`: every document in one response, for an agent that wants a single fetch.
#[derive(Template)]
#[template(path = "llms-full.md")]
pub struct LlmsFull {
    pub title: &'static str,
    pub summary: &'static str,
    pub documents: Vec<FullDocument>,
}

impl LlmsFull {
    pub fn build(
        catalog: &Catalog,
        base_url: &BaseUrl,
    ) -> LlmsFull {
        LlmsFull {
            title: SITE_TITLE,
            summary: SITE_SUMMARY,
            documents: catalog.list_documents().map(|document| FullDocument::build(document, base_url)).collect(),
        }
    }
}

impl FullDocument {
    fn build(
        document: &Document,
        base_url: &BaseUrl,
    ) -> FullDocument {
        FullDocument {
            url: base_url.to_absolute(&document.path.to_url_path()),
            body: document.body.clone(),
        }
    }
}
