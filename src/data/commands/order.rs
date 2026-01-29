use chrono::Utc;
use serde::Serialize;
use surrealdb::sql::Datetime;

use crate::{
    data::errors::DataError,
    db::DB,
    models::{
        order::{Order, PaymentStatus},
        OrderId, UserId,
    },
};

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

#[derive(Serialize)]
struct OrderData {
    user: surrealdb::RecordId,
    user_email: String,
    filename: String,
    file_size: i32,
    text_content: String,
    text_length: i32,
    price_amount: i32,
    payment_status: String,
    order_number: String,
}

pub async fn create_order(params: CreateOrderParams) -> Result<Order, DataError> {
    let order: Option<Order> = DB
        .create("order")
        .content(OrderData {
            user: params.user_id.into_record_id(),
            user_email: params.user_email,
            filename: params.filename,
            file_size: params.file_size,
            text_content: params.text_content,
            text_length: params.text_length,
            price_amount: params.price_amount,
            payment_status: PaymentStatus::Pending.as_str().to_string(),
            order_number: params.order_number,
        })
        .await?;

    Ok(order.expect("Order should be created"))
}

pub async fn update_order_payment(
    order_id: &OrderId,
    payment_key: &str,
    payment_status: PaymentStatus,
) -> Result<Order, DataError> {
    let paid_at: Option<Datetime> = if payment_status == PaymentStatus::Paid {
        Some(Datetime::from(Utc::now()))
    } else {
        None
    };

    let mut result = DB
        .query(
            "UPDATE $order SET payment_key = $payment_key, payment_status = $payment_status, paid_at = $paid_at
             WHERE payment_status = 'pending'
             RETURN *",
        )
        .bind(("order", order_id.clone().into_record_id()))
        .bind(("payment_key", payment_key.to_string()))
        .bind(("payment_status", payment_status.as_str().to_string()))
        .bind(("paid_at", paid_at))
        .await?;

    let order: Option<Order> = result.take(0)?;
    Ok(order.expect("Order should be updated"))
}
