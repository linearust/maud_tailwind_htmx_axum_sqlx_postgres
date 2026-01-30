use chrono::Utc;
use serde::Serialize;
use surrealdb::sql::Datetime;

use crate::{
    constants::{errors, payment},
    data::errors::DataError,
    db::DB,
    models::{
        order::{Order, PaymentStatus},
        OrderId, OrderNumber, UserId,
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
    pub order_number: OrderNumber,
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
    payment_status: PaymentStatus,
    order_number: OrderNumber,
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
            payment_status: PaymentStatus::Pending,
            order_number: params.order_number,
        })
        .await?;

    Ok(order.ok_or(DataError::CreationFailed(errors::ORDER_CREATION_FAILED))?)
}

#[derive(Serialize)]
struct TossPaymentConfirmationRequest {
    #[serde(rename = "paymentKey")]
    payment_key: String,
    #[serde(rename = "orderId")]
    order_id: String,
    amount: i32,
}

pub struct ConfirmPaymentParams {
    pub secret_key: String,
    pub order_number: OrderNumber,
    pub payment_key: String,
    pub amount: i32,
}

pub async fn confirm_payment_with_toss(params: ConfirmPaymentParams) -> PaymentStatus {
    let confirm_request = TossPaymentConfirmationRequest {
        payment_key: params.payment_key,
        order_id: params.order_number.to_string(),
        amount: params.amount,
    };

    let response = reqwest::Client::new()
        .post(payment::TOSS_API_CONFIRM_URL)
        .basic_auth(&params.secret_key, Some(""))
        .json(&confirm_request)
        .send()
        .await;

    match response {
        Ok(resp) if resp.status().is_success() => PaymentStatus::Paid,
        Ok(resp) => {
            let error_body = match resp.text().await {
                Ok(body) => body,
                Err(e) => {
                    tracing::error!("Failed to decode Toss API error response: {}", e);
                    "Failed to decode response".to_string()
                }
            };
            tracing::error!("Toss payment confirmation failed: {}", error_body);
            PaymentStatus::Failed
        }
        Err(e) => {
            tracing::error!("Failed to call Toss API: {}", e);
            PaymentStatus::Failed
        }
    }
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
    order.ok_or(DataError::NotFound(errors::ORDER_NOT_FOUND_OR_PROCESSED))
}
