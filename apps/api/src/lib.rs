//! The agents REST API.
//!
//! The binary in `main.rs` is a thin entrypoint over this library, so everything here is reachable from `tests/`.

pub mod content;
pub mod environment;
mod problem_list;
pub mod routes;
pub mod server;
pub mod state;
pub mod views;
