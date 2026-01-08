use time::OffsetDateTime;
use crate::models::{order::PaymentStatus, UserId};

pub use crate::models::pagination::PaginatedResult;

pub struct AdminStats {
    pub total_users: i64,
    pub total_orders: i64,
    pub total_revenue: i32,
    pub orders_last_7_days: i64,
}

pub struct UserListItem {
    pub user_id: UserId,
    pub email: String,
    pub is_admin: bool,
    pub created_at: OffsetDateTime,
    pub order_count: i64,
    pub total_spent: i32,
}

pub type UserDetail = UserListItem;

pub struct OrderListItem {
    pub order_id: String,
    pub order_number: String,
    pub user_email: String,
    pub price_amount: i32,
    pub payment_status: PaymentStatus,
    pub created_at: OffsetDateTime,
}

pub struct OrderDetail {
    pub order_id: String,
    pub order_number: String,
    pub user_id: UserId,
    pub user_email: String,
    pub price_amount: i32,
    pub payment_status: PaymentStatus,
    pub created_at: OffsetDateTime,
    pub paid_at: Option<OffsetDateTime>,
    pub payment_key: Option<String>,
    pub filename: String,
    pub text_length: i32,
}
