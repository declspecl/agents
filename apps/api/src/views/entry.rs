use crate::content::Document;
use crate::environment::BaseUrl;

/// One link in an index.
pub struct Entry {
    pub title: String,
    pub url: String,
    pub summary: String,
}

impl Entry {
    /// A link to `document`, with an absolute URL when `base_url` is given and a site-relative path otherwise.
    pub fn build(
        document: &Document,
        base_url: Option<&BaseUrl>,
    ) -> Entry {
        let url_path = document.path.to_url_path();

        Entry {
            title: document.title.clone(),
            url: base_url.map_or(url_path.clone(), |base_url| base_url.to_absolute(&url_path)),
            summary: document.summary.clone(),
        }
    }
}
