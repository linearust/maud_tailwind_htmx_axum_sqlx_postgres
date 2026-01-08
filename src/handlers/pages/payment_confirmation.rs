use axum::{Extension, extract::{Path, State}};
use maud::Markup;
use sqlx::PgPool;

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
    State(db): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
    Path(order_id): Path<OrderId>,
) -> Result<Markup, HandlerError> {
    let user_id = current_user.require_authenticated();

    let order = queries::order::get_order_for_user(&db, order_id, user_id).await?;

    if !matches!(order.payment_status, PaymentStatus::Paid) {
        return Err(DataError::Unauthorized(errors::PAYMENT_NOT_COMPLETED).into());
    }

    Ok(pages::payment_confirmation(&current_user, flash.as_ref(), config.site_name(), &order))
}
