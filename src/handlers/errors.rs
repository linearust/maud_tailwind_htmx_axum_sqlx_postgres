use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::{auth::CurrentUser, constants::error_pages, data::errors::DataError, views::pages};

pub type HandlerResult<T = Response> = Result<T, HandlerError>;

/// Wraps data/session errors and converts them to HTTP responses.
#[derive(Error, Debug)]
pub enum HandlerError {
    #[error("{0}")]
    Data(#[from] DataError),

    #[error("{0}")]
    Session(#[from] tower_sessions::session::Error),
}

impl IntoResponse for HandlerError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            Self::Data(DataError::NotFound(msg)) => (StatusCode::NOT_FOUND, *msg),
            Self::Data(DataError::Unauthorized(msg)) => (StatusCode::UNAUTHORIZED, *msg),
            Self::Data(DataError::InvalidInput(msg)) => (StatusCode::BAD_REQUEST, msg.as_str()),
            Self::Data(DataError::Database(e)) => {
                tracing::error!(error = %e, "Database error in handler");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            Self::Session(e) => {
                tracing::error!(error = %e, "Session error in handler");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };

        (status, pages::server_error(&CurrentUser::Guest, None, error_pages::FALLBACK_SITE_NAME, message)).into_response()
    }
}
