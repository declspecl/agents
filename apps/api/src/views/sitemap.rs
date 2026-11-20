use askama::Template;

use crate::content::Catalog;
use crate::environment::BaseUrl;

/// `/sitemap.xml`: the URL of every directory and document.
#[derive(Template)]
#[template(path = "sitemap.xml")]
pub struct Sitemap {
    pub urls: Vec<String>,
}

impl Sitemap {
    pub fn build(
        catalog: &Catalog,
        base_url: &BaseUrl,
    ) -> Sitemap {
        let directories = catalog.list_directories().into_iter().map(|directory| directory.to_url_path());
        let documents = catalog.list_documents().map(|document| document.path.to_url_path());

        Sitemap {
            urls: directories.chain(documents).map(|url_path| base_url.to_absolute(&url_path)).collect(),
        }
    }
}
