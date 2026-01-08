use sqlx::PgPool;

use crate::{data::errors::DataError, models::{OrderId, order::{Order, PaymentStatus}, UserId}};

pub struct CreateOrderParams {
    pub user_id: UserId,
    pub user_email: String,
    pub filename: String,
    pub file_size: i32,
    pub text_content: String,
    pub text_length: i32,
    pub price_amount: i32,
    pub order_number: String,
}

pub async fn create_order(db: &PgPool, params: CreateOrderParams) -> Result<Order, DataError> {
    let order = sqlx::query_as!(
        Order,
        r#"
        INSERT INTO orders(user_id, user_email, filename, file_size, text_content, text_length, price_amount, payment_status, order_number)
        VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING
            order_id,
            user_id,
            user_email,
            filename,
            file_size,
            text_content,
            text_length,
            price_amount,
            payment_status as "payment_status: PaymentStatus",
            payment_key,
            order_number,
            created_at,
            paid_at
        "#,
        params.user_id.as_i32(),
        params.user_email,
        params.filename,
        params.file_size,
        params.text_content,
        params.text_length,
        params.price_amount,
        PaymentStatus::Pending as PaymentStatus,
        params.order_number
    )
    .fetch_one(db)
    .await?;

    Ok(order)
}

pub async fn update_order_payment(
    db: &PgPool,
    order_id: OrderId,
    payment_key: &str,
    payment_status: PaymentStatus,
) -> Result<Order, DataError> {
    let order = sqlx::query_as!(
        Order,
        r#"
        UPDATE orders
        SET payment_key = $2, payment_status = $3, paid_at = CASE WHEN $3 = 'paid' THEN NOW() ELSE paid_at END
        WHERE order_id = $1 AND payment_status = 'pending'
        RETURNING
            order_id,
            user_id,
            user_email,
            filename,
            file_size,
            text_content,
            text_length,
            price_amount,
            payment_status as "payment_status: PaymentStatus",
            payment_key,
            order_number,
            created_at,
            paid_at
        "#,
        order_id.as_uuid(),
        payment_key,
        payment_status as PaymentStatus
    )
    .fetch_one(db)
    .await?;

    Ok(order)
}
