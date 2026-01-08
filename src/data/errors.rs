use thiserror::Error;

/// Separates DB errors from semantic errors (not found, unauthorized).
#[derive(Error, Debug)]
pub enum DataError {
    #[error("Database error")]
    Database(#[from] sqlx::Error),

    #[error("{0}")]
    NotFound(&'static str),

    #[error("{0}")]
    Unauthorized(&'static str),

    #[error("{0}")]
    InvalidInput(String),
}
