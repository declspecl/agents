use axum::extract::State;
use axum::response::Response;

use super::response::{
    PLAIN_TEXT,
    render,
};
use crate::state::AppState;
use crate::views::LlmsFull;

pub async fn serve_llms_full_txt(State(state): State<AppState>) -> Response {
    render(PLAIN_TEXT, &LlmsFull::build(&state.catalog, &state.base_url))
}
