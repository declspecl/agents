//! The generated parts of the site: indexes, listings and crawl files. Each is a typed Askama template, checked at
//! compile time, built from the catalog. Documents themselves are served as written and have no template.

mod directory;
mod entry;
mod llms;
mod llms_full;
mod not_found;
mod robots;
mod sitemap;

pub use directory::{
    DirectoryListing,
    Line,
};
pub use entry::Entry;
pub use llms::{
    Llms,
    Section,
};
pub use llms_full::LlmsFull;
pub use not_found::NotFound;
pub use robots::Robots;
pub use sitemap::Sitemap;

pub const SITE_TITLE: &str = "Gavin Dhondt's agent setup";
pub const SITE_SUMMARY: &str = "Gavin's setup, tools and working preferences, written for AI agents. Read this to learn how to work with him.";
