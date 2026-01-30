use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::{order::PaymentStatus, OrderId, OrderNumber, UserId};

pub use crate::models::pagination::PaginatedResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminStats {
    pub total_users: i64,
    pub total_orders: i64,
    pub total_revenue: i32,
    pub orders_last_7_days: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserListItem {
    pub id: UserId,
    pub email: String,
    pub is_admin: bool,
    pub created_at: DateTime<Utc>,
    pub order_count: i64,
    pub total_spent: i32,
}

pub type UserDetail = UserListItem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderListItem {
    pub id: OrderId,
    pub order_number: OrderNumber,
    pub user_email: String,
    pub price_amount: i32,
    pub payment_status: PaymentStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderDetail {
    pub id: OrderId,
    pub order_number: OrderNumber,
    pub user: UserId,
    pub user_email: String,
    pub price_amount: i32,
    pub payment_status: PaymentStatus,
    pub created_at: DateTime<Utc>,
    pub paid_at: Option<DateTime<Utc>>,
    pub payment_key: Option<String>,
    pub filename: String,
    pub text_length: i32,
}
