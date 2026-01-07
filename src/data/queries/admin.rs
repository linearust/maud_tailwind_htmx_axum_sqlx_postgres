use sqlx::PgPool;
use crate::{
    data::errors::DataError,
    models::{
        admin::{AdminStats, UserListItem, UserDetail, OrderListItem, OrderDetail},
        order::PaymentStatus,
        UserId,
    },
};

pub async fn get_admin_stats(db: &PgPool) -> Result<AdminStats, DataError> {
    let result = sqlx::query!(
        r#"
        SELECT
            (SELECT COUNT(*) FROM users) as "total_users!",
            (SELECT COUNT(*) FROM orders WHERE payment_status = 'paid') as "total_orders!",
            (SELECT COALESCE(SUM(price_amount), 0) FROM orders WHERE payment_status = 'paid') as "total_revenue!",
            (SELECT COUNT(*) FROM orders WHERE payment_status = 'paid' AND created_at >= NOW() - INTERVAL '7 days') as "orders_last_7_days!"
        "#
    )
    .fetch_one(db)
    .await?;

    Ok(AdminStats {
        total_users: result.total_users,
        total_orders: result.total_orders,
        total_revenue: result.total_revenue as i32,
        orders_last_7_days: result.orders_last_7_days,
    })
}

pub async fn get_users_paginated(
    db: &PgPool,
    page: i64,
    per_page: i64,
) -> Result<Vec<UserListItem>, DataError> {
    let offset = (page - 1) * per_page;

    let results = sqlx::query!(
        r#"
        SELECT
            u.user_id as user_id,
            u.email,
            EXISTS(SELECT 1 FROM user_roles ur WHERE ur.user_id = u.user_id AND ur.role = 'admin') as "is_admin!",
            u.created_at,
            COUNT(CASE WHEN o.payment_status = 'paid' THEN 1 END) as "order_count!",
            COALESCE(SUM(CASE WHEN o.payment_status = 'paid' THEN o.price_amount ELSE 0 END), 0) as total_spent
        FROM users u
        LEFT JOIN orders o ON u.user_id = o.user_id
        GROUP BY u.user_id, u.email, u.created_at
        ORDER BY u.created_at DESC
        LIMIT $1 OFFSET $2
        "#,
        per_page,
        offset
    )
    .fetch_all(db)
    .await?;

    Ok(results
        .into_iter()
        .map(|r| UserListItem {
            user_id: r.user_id,
            email: r.email,
            is_admin: r.is_admin,
            created_at: r.created_at,
            order_count: r.order_count,
            total_spent: r.total_spent.unwrap_or(0) as i32,
        })
        .collect())
}

pub async fn get_total_user_count(db: &PgPool) -> Result<i64, DataError> {
    let result = sqlx::query!(
        r#"
        SELECT COUNT(*) as "count!"
        FROM users
        "#
    )
    .fetch_one(db)
    .await?;

    Ok(result.count)
}

pub async fn get_user_detail(db: &PgPool, user_id: UserId) -> Result<UserDetail, DataError> {
    let result = sqlx::query!(
        r#"
        SELECT
            u.user_id as user_id,
            u.email,
            EXISTS(SELECT 1 FROM user_roles ur WHERE ur.user_id = u.user_id AND ur.role = 'admin') as "is_admin!",
            u.created_at,
            COUNT(CASE WHEN o.payment_status = 'paid' THEN 1 END) as "order_count!",
            COALESCE(SUM(CASE WHEN o.payment_status = 'paid' THEN o.price_amount ELSE 0 END), 0) as total_spent
        FROM users u
        LEFT JOIN orders o ON u.user_id = o.user_id
        WHERE u.user_id = $1
        GROUP BY u.user_id, u.email, u.created_at
        "#,
        user_id.as_i32()
    )
    .fetch_one(db)
    .await?;

    Ok(UserDetail {
        user_id: result.user_id,
        email: result.email,
        is_admin: result.is_admin,
        created_at: result.created_at,
        order_count: result.order_count,
        total_spent: result.total_spent.unwrap_or(0) as i32,
    })
}

pub async fn get_user_orders(
    db: &PgPool,
    user_id: UserId,
    page: i64,
    per_page: i64,
) -> Result<Vec<OrderListItem>, DataError> {
    let offset = (page - 1) * per_page;

    sqlx::query_as!(
        OrderListItem,
        r#"
        SELECT
            order_id::text as "order_id!",
            order_number,
            user_email,
            price_amount,
            payment_status as "payment_status: PaymentStatus",
            created_at
        FROM orders
        WHERE user_id = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
        user_id.as_i32(),
        per_page,
        offset
    )
    .fetch_all(db)
    .await
    .map_err(DataError::from)
}

pub async fn get_user_order_count(db: &PgPool, user_id: UserId) -> Result<i64, DataError> {
    let result = sqlx::query!(
        r#"
        SELECT COUNT(*) as "count!"
        FROM orders
        WHERE user_id = $1
        "#,
        user_id.as_i32()
    )
    .fetch_one(db)
    .await?;

    Ok(result.count)
}

pub async fn get_orders_paginated(
    db: &PgPool,
    status_filter: Option<PaymentStatus>,
    page: i64,
    per_page: i64,
) -> Result<Vec<OrderListItem>, DataError> {
    let offset = (page - 1) * per_page;

    let result = match status_filter {
        Some(status) => {
            sqlx::query_as!(
                OrderListItem,
                r#"
                SELECT
                    order_id::text as "order_id!",
                    order_number,
                    user_email,
                    price_amount,
                    payment_status as "payment_status: PaymentStatus",
                    created_at
                FROM orders
                WHERE payment_status = $1
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#,
                status as PaymentStatus,
                per_page,
                offset
            )
            .fetch_all(db)
            .await
        }
        None => {
            sqlx::query_as!(
                OrderListItem,
                r#"
                SELECT
                    order_id::text as "order_id!",
                    order_number,
                    user_email,
                    price_amount,
                    payment_status as "payment_status: PaymentStatus",
                    created_at
                FROM orders
                ORDER BY created_at DESC
                LIMIT $1 OFFSET $2
                "#,
                per_page,
                offset
            )
            .fetch_all(db)
            .await
        }
    };

    result.map_err(DataError::from)
}

pub async fn get_total_order_count(
    db: &PgPool,
    status_filter: Option<PaymentStatus>,
) -> Result<i64, DataError> {
    match status_filter {
        Some(status) => {
            let result = sqlx::query!(
                r#"
                SELECT COUNT(*) as "count!"
                FROM orders
                WHERE payment_status = $1
                "#,
                status as PaymentStatus
            )
            .fetch_one(db)
            .await?;

            Ok(result.count)
        }
        None => {
            let result = sqlx::query!(
                r#"
                SELECT COUNT(*) as "count!"
                FROM orders
                "#
            )
            .fetch_one(db)
            .await?;

            Ok(result.count)
        }
    }
}

pub async fn get_order_detail(db: &PgPool, order_id: &str) -> Result<OrderDetail, DataError> {
    let uuid_order_id = sqlx::types::Uuid::parse_str(order_id)
        .map_err(|_| DataError::NotFound("Invalid order ID format"))?;

    sqlx::query_as!(
        OrderDetail,
        r#"
        SELECT
            order_id::text as "order_id!",
            order_number,
            user_id,
            user_email,
            price_amount,
            payment_status as "payment_status: PaymentStatus",
            created_at,
            paid_at,
            payment_key,
            filename,
            text_length
        FROM orders
        WHERE order_id = $1
        "#,
        uuid_order_id
    )
    .fetch_one(db)
    .await
    .map_err(DataError::from)
}
