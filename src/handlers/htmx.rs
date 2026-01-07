//! HTMX response helpers for consistent interaction patterns.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use maud::Markup;

pub fn no_content() -> Response {
    StatusCode::OK.into_response()
}

pub fn html_fragment(markup: Markup) -> Response {
    markup.into_response()
}
