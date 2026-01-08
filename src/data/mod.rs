//! CQRS pattern: commands (mutations) and queries (reads).

pub mod commands;
pub mod errors;
pub mod queries;

use errors::DataError;
use sqlx::postgres::PgQueryResult;

/// Validates authorization: WHERE clause with user_id must affect rows.
pub fn ensure_rows_affected(result: PgQueryResult, message: &'static str) -> Result<(), DataError> {
    if result.rows_affected() == 0 {
        Err(DataError::NotFound(message))
    } else {
        Ok(())
    }
}

pub fn map_row_not_found(error: sqlx::Error, message: &'static str) -> DataError {
    match error {
        sqlx::Error::RowNotFound => DataError::NotFound(message),
        _ => DataError::Database(error),
    }
}

/// Use when missing row means authorization failure, not simple not-found.
pub fn map_row_unauthorized(error: sqlx::Error, message: &'static str) -> DataError {
    match error {
        sqlx::Error::RowNotFound => DataError::Unauthorized(message),
        _ => DataError::Database(error),
    }
}
