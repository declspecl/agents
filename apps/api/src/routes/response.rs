use askama::Template;
use axum::http::{
    HeaderValue,
    StatusCode,
    header,
};
use axum::response::{
    IntoResponse,
    Response,
};

pub const MARKDOWN: &str = "text/markdown; charset=utf-8";
pub const PLAIN_TEXT: &str = "text/plain; charset=utf-8";
pub const XML: &str = "application/xml; charset=utf-8";

/// A response with `body` as the given content type.
pub fn respond(
    status: StatusCode,
    content_type: &'static str,
    body: String,
) -> Response {
    (status, [(header::CONTENT_TYPE, HeaderValue::from_static(content_type))], body).into_response()
}

/// Renders `template` as a 200 response. A template that fails to render is a bug, so it becomes a 500.
pub fn render(
    content_type: &'static str,
    template: &impl Template,
) -> Response {
    render_with_status(StatusCode::OK, content_type, template)
}

pub fn render_with_status(
    status: StatusCode,
    content_type: &'static str,
    template: &impl Template,
) -> Response {
    match template.render() {
        Ok(body) => respond(status, content_type, body),
        Err(error) => {
            eprintln!("template failed to render: {error}");
            respond(StatusCode::INTERNAL_SERVER_ERROR, PLAIN_TEXT, "internal error\n".to_owned())
        },
    }
}
