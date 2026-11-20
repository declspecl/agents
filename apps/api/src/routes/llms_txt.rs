use axum::extract::State;
use axum::response::Response;

use super::response::{
    PLAIN_TEXT,
    render,
};
use crate::state::AppState;
use crate::views::Llms;

pub async fn serve_llms_txt(State(state): State<AppState>) -> Response {
    render(PLAIN_TEXT, &Llms::build(&state.catalog, &state.base_url))
}
