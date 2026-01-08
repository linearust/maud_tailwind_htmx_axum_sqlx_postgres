use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Strongly-typed user identifier.
///
/// Wraps a database user_id to prevent accidental confusion with other i32 values
/// (like todo_id, price_amount, etc.). This is the "parse, don't validate" pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct UserId(i32);

impl UserId {
    /// Only call when reading from database or session — DB generates IDs for new users.
    pub fn from_db(id: i32) -> Self {
        Self(id)
    }

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
    /// Only call when reading from database — DB generates IDs for new todos.
    pub fn from_db(id: i32) -> Self {
        Self(id)
    }

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

/// Strongly-typed order identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct OrderId(Uuid);

impl OrderId {
    #[allow(dead_code)]
    pub fn from_db(id: Uuid) -> Self {
        Self(id)
    }

    pub fn as_uuid(self) -> Uuid {
        self.0
    }

    #[allow(dead_code)]
    pub fn parse(s: &str) -> Result<Self, uuid::Error> {
        Uuid::parse_str(s).map(Self)
    }
}

impl std::fmt::Display for OrderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for OrderId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}
