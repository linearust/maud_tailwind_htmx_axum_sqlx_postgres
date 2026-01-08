use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Macro for strongly-typed database identifiers.
/// Prevents accidental confusion between different ID types ("parse, don't validate" pattern).
macro_rules! define_id {
    ($name:ident, $inner:ty, $accessor:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
        #[sqlx(transparent)]
        pub struct $name($inner);

        impl $name {
            /// Only call when reading from database or session — DB generates IDs.
            #[allow(dead_code)]
            pub fn from_db(id: $inner) -> Self {
                Self(id)
            }

            pub fn $accessor(self) -> $inner {
                self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<$inner> for $name {
            fn from(id: $inner) -> Self {
                Self(id)
            }
        }
    };
}

define_id!(UserId, i32, as_i32);
define_id!(TodoId, i32, as_i32);
define_id!(OrderId, Uuid, as_uuid);

impl OrderId {
    #[allow(dead_code)]
    pub fn parse(s: &str) -> Result<Self, uuid::Error> {
        Uuid::parse_str(s).map(Self)
    }
}
