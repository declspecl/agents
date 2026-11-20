mod health;

use axum::Router;
use axum::routing::get;

pub fn build_router() -> Router {
    Router::new().route("/health", get(health::check_health))
}
