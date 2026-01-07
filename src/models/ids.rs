use serde::{Deserialize, Serialize};

/// Strongly-typed user identifier.
///
/// Wraps a database user_id to prevent accidental confusion with other i32 values
/// (like todo_id, price_amount, etc.). This is the "parse, don't validate" pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct UserId(i32);

impl UserId {
    /// Creates a new UserId from a raw database value.
    ///
    /// This should only be called when reading from the database or session.
    /// For new users, the database generates the ID.
    pub fn from_db(id: i32) -> Self {
        Self(id)
    }

    /// Returns the inner value for database operations.
    pub fn as_i32(self) -> i32 {
        self.0
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i32> for UserId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

/// Strongly-typed todo identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct TodoId(i32);

impl TodoId {
    /// Creates a new TodoId from a raw database value.
    ///
    /// This should only be called when reading from the database.
    /// For new todos, the database generates the ID.
    pub fn from_db(id: i32) -> Self {
        Self(id)
    }

    /// Returns the inner value for database operations.
    pub fn as_i32(self) -> i32 {
        self.0
    }
}

impl std::fmt::Display for TodoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i32> for TodoId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}
