use std::sync::Arc;

use crate::content::Catalog;
use crate::environment::BaseUrl;

/// What every request handler shares. Cheap to clone: the catalog is behind an `Arc`.
#[derive(Clone)]
pub struct AppState {
    pub catalog: Arc<Catalog>,
    pub base_url: BaseUrl,
}

impl AppState {
    pub fn new(
        catalog: Catalog,
        base_url: BaseUrl,
    ) -> AppState {
        AppState {
            catalog: Arc::new(catalog),
            base_url,
        }
    }
}
