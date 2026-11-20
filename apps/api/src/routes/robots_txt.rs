use axum::extract::State;
use axum::response::Response;

use super::response::{
    PLAIN_TEXT,
    render,
};
use crate::state::AppState;
use crate::views::Robots;

pub async fn serve_robots_txt(State(state): State<AppState>) -> Response {
    render(PLAIN_TEXT, &Robots::build(&state.base_url))
}
