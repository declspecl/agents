use axum::extract::State;
use axum::response::Response;

use super::response::{
    XML,
    render,
};
use crate::state::AppState;
use crate::views::Sitemap;

pub async fn serve_sitemap_xml(State(state): State<AppState>) -> Response {
    render(XML, &Sitemap::build(&state.catalog, &state.base_url))
}
