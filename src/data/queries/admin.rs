use chrono::{DateTime, Utc};
use serde::Deserialize;
use surrealdb::RecordId;

use crate::{
    constants::errors,
    data::errors::DataError,
    db::DB,
    models::{
        admin::{AdminStats, OrderDetail, OrderListItem, UserDetail, UserListItem},
        order::PaymentStatus,
        pagination, OrderId, UserId,
    },
};

use super::shared::{check_user_is_admin, CountResult, SumResult};

// ───────────────────────────────────────────────────────────────────────────────
// Private Query Helpers
// ───────────────────────────────────────────────────────────────────────────────

struct UserOrderStats {
    order_count: i64,
    total_spent: i32,
}

async fn get_user_order_stats(user_record_id: &RecordId) -> Result<UserOrderStats, DataError> {
    let mut result = DB
        .query(
            r#"
            SELECT count() as count FROM order WHERE user = $user AND payment_status = $paid_status GROUP ALL;
            SELECT math::sum(price_amount) as total FROM order WHERE user = $user AND payment_status = $paid_status GROUP ALL;
            "#,
        )
        .bind(("user", user_record_id.clone()))
        .bind(("paid_status", PaymentStatus::Paid.as_str()))
        .await?;

    let order_count: Option<CountResult> = result.take(0)?;
    let total_spent: Option<SumResult> = result.take(1)?;

    Ok(UserOrderStats {
        order_count: CountResult::unwrap_or_zero(order_count),
        total_spent: SumResult::unwrap_or_zero(total_spent) as i32,
    })
}

pub async fn get_admin_stats() -> Result<AdminStats, DataError> {
    let mut result = DB
        .query(
            r#"
            SELECT count() as count FROM user GROUP ALL;
            SELECT count() as count FROM order WHERE payment_status = $paid_status GROUP ALL;
            SELECT math::sum(price_amount) as total FROM order WHERE payment_status = $paid_status GROUP ALL;
            SELECT count() as count FROM order WHERE payment_status = $paid_status AND created_at >= time::now() - 7d GROUP ALL;
            "#,
        )
        .bind(("paid_status", PaymentStatus::Paid.as_str()))
        .await?;

    let total_users: Option<CountResult> = result.take(0)?;
    let total_orders: Option<CountResult> = result.take(1)?;
    let total_revenue: Option<SumResult> = result.take(2)?;
    let orders_last_7_days: Option<CountResult> = result.take(3)?;

    Ok(AdminStats {
        total_users: CountResult::unwrap_or_zero(total_users),
        total_orders: CountResult::unwrap_or_zero(total_orders),
        total_revenue: SumResult::unwrap_or_zero(total_revenue) as i32,
        orders_last_7_days: CountResult::unwrap_or_zero(orders_last_7_days),
    })
}

#[derive(Deserialize)]
struct UserWithStats {
    id: UserId,
    email: String,
    created_at: DateTime<Utc>,
}

pub async fn get_users_paginated(page: i64, per_page: i64) -> Result<Vec<UserListItem>, DataError> {
    let offset = pagination::offset(page, per_page);

    let mut result = DB
        .query(
            "SELECT id, email, created_at FROM user ORDER BY created_at DESC LIMIT $limit START $offset",
        )
        .bind(("limit", per_page))
        .bind(("offset", offset))
        .await?;

    let users: Vec<UserWithStats> = result.take(0)?;

    let mut items = Vec::with_capacity(users.len());
    for user in users {
        let user_record_id = user.id.clone().into_record_id();
        let is_admin = check_user_is_admin(&user_record_id).await?;
        let stats = get_user_order_stats(&user_record_id).await?;

        items.push(UserListItem {
            id: user.id,
            email: user.email,
            is_admin,
            created_at: user.created_at,
            order_count: stats.order_count,
            total_spent: stats.total_spent,
        });
    }

    Ok(items)
}

pub async fn get_total_user_count() -> Result<i64, DataError> {
    let mut result = DB
        .query("SELECT count() as count FROM user GROUP ALL")
        .await?;

    let count: Option<CountResult> = result.take(0)?;
    Ok(CountResult::unwrap_or_zero(count))
}

pub async fn get_user_detail(user_id: &UserId) -> Result<UserDetail, DataError> {
    let user: Option<UserWithStats> = DB.select(user_id.clone().into_record_id()).await?;
    let user = user.ok_or(DataError::NotFound(errors::USER_NOT_FOUND))?;

    let user_record_id = user_id.clone().into_record_id();
    let is_admin = check_user_is_admin(&user_record_id).await?;
    let stats = get_user_order_stats(&user_record_id).await?;

    Ok(UserDetail {
        id: user.id,
        email: user.email,
        is_admin,
        created_at: user.created_at,
        order_count: stats.order_count,
        total_spent: stats.total_spent,
    })
}

pub async fn get_user_orders(
    user_id: &UserId,
    page: i64,
    per_page: i64,
) -> Result<Vec<OrderListItem>, DataError> {
    let offset = pagination::offset(page, per_page);

    let mut result = DB
        .query(
            "SELECT id, order_number, user_email, price_amount, payment_status, created_at
             FROM order
             WHERE user = $user
             ORDER BY created_at DESC
             LIMIT $limit START $offset",
        )
        .bind(("user", user_id.clone().into_record_id()))
        .bind(("limit", per_page))
        .bind(("offset", offset))
        .await?;

    let orders: Vec<OrderListItem> = result.take(0)?;
    Ok(orders)
}

pub async fn get_user_order_count(user_id: &UserId) -> Result<i64, DataError> {
    let mut result = DB
        .query("SELECT count() as count FROM order WHERE user = $user GROUP ALL")
        .bind(("user", user_id.clone().into_record_id()))
        .await?;

    let count: Option<CountResult> = result.take(0)?;
    Ok(CountResult::unwrap_or_zero(count))
}

fn build_status_filter_clause(status_filter: &Option<PaymentStatus>) -> &'static str {
    match status_filter {
        Some(_) => "WHERE payment_status = $status",
        None => "",
    }
}

pub async fn get_orders_paginated(
    status_filter: Option<PaymentStatus>,
    page: i64,
    per_page: i64,
) -> Result<Vec<OrderListItem>, DataError> {
    let offset = pagination::offset(page, per_page);
    let where_clause = build_status_filter_clause(&status_filter);

    let query = format!(
        "SELECT id, order_number, user_email, price_amount, payment_status, created_at
         FROM order
         {where_clause}
         ORDER BY created_at DESC
         LIMIT $limit START $offset"
    );

    let mut result = DB
        .query(&query)
        .bind(("status", status_filter.map(|s| s.as_str().to_string())))
        .bind(("limit", per_page))
        .bind(("offset", offset))
        .await?;

    let orders: Vec<OrderListItem> = result.take(0)?;
    Ok(orders)
}

pub async fn get_total_order_count(status_filter: Option<PaymentStatus>) -> Result<i64, DataError> {
    let where_clause = build_status_filter_clause(&status_filter);

    let query = format!("SELECT count() as count FROM order {where_clause} GROUP ALL");

    let mut result = DB
        .query(&query)
        .bind(("status", status_filter.map(|s| s.as_str().to_string())))
        .await?;

    let count: Option<CountResult> = result.take(0)?;
    Ok(CountResult::unwrap_or_zero(count))
}

pub async fn get_order_detail(order_id: &OrderId) -> Result<OrderDetail, DataError> {
    let mut result = DB
        .query(
            "SELECT id, order_number, user, user_email, price_amount, payment_status, created_at, paid_at, payment_key, filename, text_length
             FROM order
             WHERE id = $order_id",
        )
        .bind(("order_id", order_id.clone().into_record_id()))
        .await?;

    let order: Option<OrderDetail> = result.take(0)?;
    order.ok_or(DataError::NotFound(errors::ORDER_NOT_FOUND))
}
