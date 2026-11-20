use askama::Template;

/// The body of every 404, which points a lost agent back at the index.
#[derive(Template)]
#[template(path = "not-found.md")]
pub struct NotFound {
    pub url_path: String,
}

impl NotFound {
    pub fn build(url_path: &str) -> NotFound {
        NotFound {
            url_path: url_path.to_owned(),
        }
    }
}
