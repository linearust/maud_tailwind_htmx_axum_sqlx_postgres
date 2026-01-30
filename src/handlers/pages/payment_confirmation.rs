use axum::{Extension, extract::{Path, State}};
use maud::Markup;

use crate::{
    auth::CurrentUser,
    config::AppConfig,
    constants::errors,
    data::{errors::DataError, queries},
    handlers::errors::HandlerError,
    models::{order::PaymentStatus, OrderId},
    session::FlashMessage,
    views::pages,
};

pub async fn get_payment_confirmation(
    State(config): State<AppConfig>,
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
    Path(raw_order_id): Path<String>,
) -> Result<Markup, HandlerError> {
    let user_id = current_user.require_authenticated()?;
    let order_id = OrderId::parse_or_not_found(&raw_order_id, errors::ORDER_NOT_FOUND)?;

    let order = queries::order::get_order_for_user(&order_id, user_id).await?;

    if !matches!(order.payment_status, PaymentStatus::Paid) {
        return Err(DataError::Unauthorized(errors::PAYMENT_NOT_COMPLETED).into());
    }

    Ok(pages::payment_confirmation(&current_user, flash.as_ref(), config.site_name(), &order))
}
