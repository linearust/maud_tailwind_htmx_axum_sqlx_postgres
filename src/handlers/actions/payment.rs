use axum::{Extension, Form, extract::{Query, State}, response::{IntoResponse, Redirect}};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

use crate::{
    auth::CurrentUser,
    config::AppConfig,
    constants::messages,
    data::{commands, queries},
    session::FlashMessage,
    handlers::errors::HandlerResult,
    models::{OrderId, order::PaymentStatus},
    paths,
};

#[derive(Deserialize)]
pub struct PaymentInitiateForm {
    order_id: OrderId,
}

pub async fn post_actions_payment_initiate(
    Extension(current_user): Extension<CurrentUser>,
    session: Session,
    Form(form): Form<PaymentInitiateForm>,
) -> HandlerResult {
    let user_id = current_user.require_authenticated();

    let order = queries::order::get_order_for_user(&form.order_id, user_id).await?;

    if !matches!(order.payment_status, PaymentStatus::Pending) {
        return Ok(FlashMessage::error(messages::ORDER_ALREADY_PROCESSED)
            .set_and_redirect(&session, &paths::helpers::quote_path(&order.id))
            .await?);
    }

    let checkout_url = paths::helpers::checkout_path(&order.id);
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
struct TossPaymentConfirmationRequest {
    #[serde(rename = "paymentKey")]
    payment_key: String,
    #[serde(rename = "orderId")]
    order_id: String,
    amount: i32,
}


async fn confirm_payment_with_toss(secret_key: &str, query: &PaymentVerifyQuery) -> PaymentStatus {
    let confirm_request = TossPaymentConfirmationRequest {
        payment_key: query.payment_key.clone(),
        order_id: query.order_id.clone(),
        amount: query.amount,
    };

    let response = reqwest::Client::new()
        .post(crate::constants::payment::TOSS_API_CONFIRM_URL)
        .basic_auth(secret_key, Some(""))
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

async fn redirect_with_error(session: &Session, order_id: &OrderId) -> HandlerResult {
    Ok(FlashMessage::error(messages::PAYMENT_FAILED)
        .set_and_redirect(session, &paths::helpers::quote_path(order_id))
        .await?)
}

pub async fn get_actions_payment_verify(
    State(config): State<AppConfig>,
    Extension(current_user): Extension<CurrentUser>,
    session: Session,
    Query(query): Query<PaymentVerifyQuery>,
) -> HandlerResult {
    let user_id = current_user.require_authenticated();
    let order = queries::order::get_order_by_order_number_for_user(&query.order_id, user_id).await?;

    if query.amount != order.price_amount {
        tracing::error!("Payment amount mismatch: expected {}, got {}", order.price_amount, query.amount);
        return redirect_with_error(&session, &order.id).await;
    }

    let status = confirm_payment_with_toss(config.payment().toss_secret_key(), &query).await;

    commands::order::update_order_payment(&order.id, &query.payment_key, status).await?;

    match status {
        PaymentStatus::Paid => {
            Ok(FlashMessage::success(messages::PAYMENT_SUCCESS)
                .set_and_redirect(&session, &paths::helpers::payment_confirmation_path(&order.id))
                .await?)
        }
        _ => redirect_with_error(&session, &order.id).await,
    }
}
