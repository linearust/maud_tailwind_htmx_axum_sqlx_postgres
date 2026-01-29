use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{OrderId, UserId};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PaymentStatus {
    Pending,
    Paid,
    Failed,
    Cancelled,
}

impl PaymentStatus {
    pub fn display_text(&self) -> &'static str {
        match self {
            Self::Paid => "Paid",
            Self::Pending => "Pending",
            Self::Failed => "Failed",
            Self::Cancelled => "Cancelled",
        }
    }

    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Paid => "text-green-600",
            Self::Pending => "text-yellow-600",
            Self::Failed => "text-red-600",
            Self::Cancelled => "text-gray-600",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Paid => "paid",
            Self::Pending => "pending",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: OrderId,
    pub user: UserId,
    pub user_email: String,
    pub filename: String,
    pub file_size: i32,
    pub text_content: String,
    pub text_length: i32,
    pub price_amount: i32,
    pub payment_status: PaymentStatus,
    pub payment_key: Option<String>,
    pub order_number: String,
    pub created_at: DateTime<Utc>,
    pub paid_at: Option<DateTime<Utc>>,
}

impl Order {
    pub fn generate_order_number(user_id: &UserId) -> String {
        let uuid_string = Uuid::new_v4().to_string();
        let uuid_prefix = uuid_string
            .split('-')
            .next()
            .expect("UUID should have at least one segment");
        format!("ORD-{}-{}", user_id, uuid_prefix)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderSummary {
    pub id: OrderId,
    pub filename: String,
    pub file_size: i32,
    pub text_length: i32,
    pub price_amount: i32,
    pub payment_status: PaymentStatus,
    pub order_number: String,
    pub created_at: DateTime<Utc>,
}
