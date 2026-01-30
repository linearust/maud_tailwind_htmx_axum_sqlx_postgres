use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::UserId;

/// Strongly-typed order number for payment and order identification.
/// Format: ORD-{user_id}-{uuid_prefix}
/// Distinct from OrderId (database record ID) — OrderNumber is human-readable
/// and used in payment flows.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OrderNumber(String);

impl OrderNumber {
    pub fn generate(user_id: &UserId) -> Self {
        let uuid_string = Uuid::new_v4().to_string();
        let uuid_prefix = uuid_string
            .split('-')
            .next()
            .expect("UUID should have at least one segment");
        Self(format!("ORD-{}-{}", user_id, uuid_prefix))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for OrderNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for OrderNumber {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<OrderNumber> for String {
    fn from(order_number: OrderNumber) -> Self {
        order_number.0
    }
}
