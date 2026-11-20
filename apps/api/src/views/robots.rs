use askama::Template;

use crate::environment::BaseUrl;

/// `/robots.txt`: every agent may read everything, and these are the maps of the site.
#[derive(Template)]
#[template(path = "robots.txt")]
pub struct Robots {
    pub sitemap_url: String,
    pub llms_url: String,
}

impl Robots {
    pub fn build(base_url: &BaseUrl) -> Robots {
        Robots {
            sitemap_url: base_url.to_absolute("/sitemap.xml"),
            llms_url: base_url.to_absolute("/llms.txt"),
        }
    }
}
