use std::collections::BTreeMap;

use askama::Template;

use super::{
    Entry,
    SITE_SUMMARY,
    SITE_TITLE,
};
use crate::content::Catalog;
use crate::environment::BaseUrl;

/// A heading and the documents under it.
pub struct Section {
    pub heading: String,
    pub entries: Vec<Entry>,
}

/// `/llms.txt`: the entry point an agent reads first. Sections follow the top-level directories.
#[derive(Template)]
#[template(path = "llms.md")]
pub struct Llms {
    pub title: &'static str,
    pub summary: &'static str,
    pub sitemap_url: String,
    pub full_url: String,
    pub sections: Vec<Section>,
}

const ROOT_HEADING: &str = "Overview";

impl Llms {
    pub fn build(
        catalog: &Catalog,
        base_url: &BaseUrl,
    ) -> Llms {
        let mut grouped: BTreeMap<String, Vec<Entry>> = BTreeMap::new();
        for document in catalog.list_documents() {
            let heading = match document.path.as_str().split_once('/') {
                Some((top, _)) => capitalize(&top.replace('-', " ")),
                None => ROOT_HEADING.to_owned(),
            };
            grouped.entry(heading).or_default().push(Entry::build(document, Some(base_url)));
        }

        // The overview comes first; every other section follows in name order.
        let overview = grouped.remove(ROOT_HEADING);
        let sections = overview
            .map(|entries| Section {
                heading: ROOT_HEADING.to_owned(),
                entries,
            })
            .into_iter()
            .chain(grouped.into_iter().map(|(heading, entries)| Section {
                heading,
                entries,
            }))
            .collect();

        Llms {
            title: SITE_TITLE,
            summary: SITE_SUMMARY,
            sitemap_url: base_url.to_absolute("/sitemap.xml"),
            full_url: base_url.to_absolute("/llms-full.txt"),
            sections,
        }
    }
}

fn capitalize(text: &str) -> String {
    let mut characters = text.chars();

    characters.next().map_or_else(String::new, |first| first.to_uppercase().chain(characters).collect())
}
