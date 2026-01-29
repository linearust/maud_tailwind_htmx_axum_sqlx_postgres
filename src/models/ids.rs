use serde::{Deserialize, Serialize};
use surrealdb::RecordIdKey;

/// Macro for strongly-typed database identifiers using SurrealDB's RecordId.
/// Prevents accidental confusion between different ID types ("parse, don't validate" pattern).
macro_rules! define_id {
    ($name:ident, $table:expr) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(surrealdb::RecordId);

        impl $name {
            pub const TABLE: &'static str = $table;

            pub fn new(id: impl Into<RecordIdKey>) -> Self {
                Self(surrealdb::RecordId::from(($table, id.into())))
            }

            /// Only call when reading from database or session — DB generates IDs.
            pub fn from_record_id(id: surrealdb::RecordId) -> Self {
                Self(id)
            }

            pub fn as_record_id(&self) -> &surrealdb::RecordId {
                &self.0
            }

            pub fn into_record_id(self) -> surrealdb::RecordId {
                self.0
            }

            pub fn key(&self) -> &RecordIdKey {
                self.0.key()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0.key())
            }
        }

        impl From<surrealdb::RecordId> for $name {
            fn from(id: surrealdb::RecordId) -> Self {
                Self(id)
            }
        }

        impl From<$name> for surrealdb::RecordId {
            fn from(id: $name) -> Self {
                id.0
            }
        }
    };
}

define_id!(UserId, "user");
define_id!(TodoId, "todo");
define_id!(OrderId, "order");

impl UserId {
    pub fn parse(s: &str) -> Option<Self> {
        if s.is_empty() {
            return None;
        }
        Some(Self::new(s.to_string()))
    }
}

impl TodoId {
    pub fn parse(s: &str) -> Option<Self> {
        if s.is_empty() {
            return None;
        }
        Some(Self::new(s.to_string()))
    }
}

impl OrderId {
    pub fn parse(s: &str) -> Option<Self> {
        if s.is_empty() {
            return None;
        }
        Some(Self::new(s.to_string()))
    }
}
