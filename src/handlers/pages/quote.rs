use axum::{Extension, extract::{Path, State}};
use maud::Markup;
use sqlx::PgPool;

use crate::{
    auth::CurrentUser,
    config::AppConfig,
    data::queries,
    session::FlashMessage,
    handlers::errors::HandlerError,
    models::OrderId,
    views::pages,
};

pub async fn get_quote(
    State(config): State<AppConfig>,
    State(db): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
    Path(order_id): Path<OrderId>,
) -> Result<Markup, HandlerError> {
    let user_id = current_user.require_authenticated();

    let order = queries::order::get_order_for_user(&db, order_id, user_id).await?;

    Ok(pages::quote(&current_user, flash.as_ref(), config.site_name(), &order))
}
