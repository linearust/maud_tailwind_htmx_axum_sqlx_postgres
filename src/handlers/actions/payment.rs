use axum::{Extension, Form, extract::{Query, State}, response::{IntoResponse, Redirect}};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    auth::CurrentUser,
    config::AppConfig,
    constants::messages,
    data::{commands, queries},
    flash::FlashMessage,
    handlers::errors::HandlerResult,
    models::order::PaymentStatus,
    paths,
};
use tower_sessions::Session;

#[derive(Deserialize)]
pub struct PaymentInitiateForm {
    order_id: Uuid,
}

pub async fn post_actions_payment_initiate(
    State(db): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    session: Session,
    Form(form): Form<PaymentInitiateForm>,
) -> HandlerResult {
    let user_id = current_user.require_authenticated();

    let order = queries::order::get_order_for_user(&db, form.order_id, user_id).await?;

    if !matches!(order.payment_status, PaymentStatus::Pending) {
        return Ok(FlashMessage::error(messages::ORDER_ALREADY_PROCESSED)
            .set_and_redirect(&session, &paths::helpers::quote_path(&order.order_id))
            .await?);
    }

    let checkout_url = paths::helpers::checkout_path(&order.order_id);
    Ok(Redirect::to(&checkout_url).into_response())
}

#[derive(Deserialize)]
pub struct PaymentVerifyQuery {
    #[serde(rename = "orderId")]
    order_id: String,
    #[serde(rename = "paymentKey")]
    payment_key: String,
    amount: i32,
}

#[derive(Serialize)]
struct TossConfirmRequest {
    #[serde(rename = "paymentKey")]
    payment_key: String,
    #[serde(rename = "orderId")]
    order_id: String,
    amount: i32,
}

pub async fn get_actions_payment_verify(
    State(config): State<AppConfig>,
    State(db): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    session: Session,
    Query(query): Query<PaymentVerifyQuery>,
) -> HandlerResult {
    let user_id = current_user.require_authenticated();

    let order = queries::order::get_order_by_order_number_for_user(&db, &query.order_id, user_id).await?;

    if query.amount != order.price_amount {
        tracing::error!("Payment amount mismatch: expected {}, got {}", order.price_amount, query.amount);
        return Ok(FlashMessage::error(messages::PAYMENT_FAILED)
            .set_and_redirect(&session, &paths::helpers::quote_path(&order.order_id))
            .await?);
    }

    let secret_key = config.payment().toss_secret_key();

    let confirm_request = TossConfirmRequest {
        payment_key: query.payment_key.clone(),
        order_id: query.order_id.clone(),
        amount: query.amount,
    };

    let client = reqwest::Client::new();
    let response = client
        .post(crate::constants::payment::TOSS_API_CONFIRM_URL)
        .basic_auth(secret_key, Some(""))
        .json(&confirm_request)
        .send()
        .await;

    match response {
        Ok(resp) if resp.status().is_success() => {
            commands::order::update_order_payment(
                &db,
                order.order_id,
                &query.payment_key,
                PaymentStatus::Paid,
            ).await?;

            Ok(FlashMessage::success(messages::PAYMENT_SUCCESS)
                .set_and_redirect(&session, &paths::helpers::result_path(&order.order_id))
                .await?)
        }
        Ok(resp) => {
            let error_body = resp.text().await.unwrap_or("Unknown error".to_string());
            tracing::error!("Toss payment confirmation failed: {}", error_body);

            commands::order::update_order_payment(
                &db,
                order.order_id,
                &query.payment_key,
                PaymentStatus::Failed,
            ).await?;

            Ok(FlashMessage::error(messages::PAYMENT_FAILED)
                .set_and_redirect(&session, &paths::helpers::quote_path(&order.order_id))
                .await?)
        }
        Err(e) => {
            tracing::error!("Failed to call Toss API: {}", e);
            Ok(FlashMessage::error(messages::PAYMENT_FAILED)
                .set_and_redirect(&session, &paths::helpers::quote_path(&order.order_id))
                .await?)
        }
    }
}
