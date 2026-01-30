use axum::{Extension, extract::{Path, State}};
use maud::Markup;

use crate::{
    auth::CurrentUser,
    config::AppConfig,
    constants::errors,
    data::queries::admin,
    session::FlashMessage,
    handlers::errors::HandlerError,
    models::OrderId,
    views::pages::admin as admin_views,
};

pub async fn get_admin_order_detail(
    State(config): State<AppConfig>,
    Path(raw_order_id): Path<String>,
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> Result<Markup, HandlerError> {
    let order_id = OrderId::parse_or_not_found(&raw_order_id, errors::ORDER_NOT_FOUND)?;
    let order = admin::get_order_detail(&order_id).await?;

    Ok(admin_views::order_detail(
        &current_user,
        flash.as_ref(),
        config.site_name(),
        order,
    ))
}
