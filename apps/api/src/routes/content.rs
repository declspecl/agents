//! The virtual filesystem: documents, directory listings, and everything that is not found.
//!
//! A path ending in `/` is a directory, a path ending in `.md` is a document, and a bare name that is a directory
//! redirects to its slash form.

use axum::extract::{
    Path,
    State,
};
use axum::http::StatusCode;
use axum::response::{
    IntoResponse,
    Redirect,
    Response,
};

use super::response::{
    MARKDOWN,
    render,
    render_with_status,
    respond,
};
use crate::content::{
    DirectoryPath,
    DocumentPath,
};
use crate::state::AppState;
use crate::views::{
    DirectoryListing,
    NotFound,
};

pub async fn serve_root(State(state): State<AppState>) -> Response {
    serve_directory(&state, &DirectoryPath::ROOT, "/")
}

pub async fn serve_path(
    State(state): State<AppState>,
    Path(path): Path<String>,
) -> Response {
    let url_path = format!("/{path}");

    if let Some(directory) = path.strip_suffix('/') {
        return match DirectoryPath::parse(directory) {
            Ok(directory) => serve_directory(&state, &directory, &url_path),
            Err(_) => serve_not_found(&url_path),
        };
    }

    if path.ends_with(".md") {
        let document = DocumentPath::parse(&path).ok().and_then(|path| state.catalog.find_document(&path));

        return match document {
            Some(document) => respond(StatusCode::OK, MARKDOWN, document.body.clone()),
            None => serve_not_found(&url_path),
        };
    }

    match DirectoryPath::parse(&path) {
        Ok(directory) if state.catalog.find_directory(&directory).is_some() => Redirect::permanent(&directory.to_url_path()).into_response(),
        _ => serve_not_found(&url_path),
    }
}

fn serve_directory(
    state: &AppState,
    directory: &DirectoryPath,
    url_path: &str,
) -> Response {
    match state.catalog.find_directory(directory) {
        Some(directory) => render(MARKDOWN, &DirectoryListing::build(&directory)),
        None => serve_not_found(url_path),
    }
}

fn serve_not_found(url_path: &str) -> Response {
    render_with_status(StatusCode::NOT_FOUND, MARKDOWN, &NotFound::build(url_path))
}
