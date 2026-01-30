use axum::{Extension, extract::State};
use maud::Markup;

use crate::{
    auth::CurrentUser,
    config::AppConfig,
    constants,
    data::queries,
    handlers::errors::HandlerError,
    session::FlashMessage,
    views::pages,
};

pub async fn get_dashboard(
    State(config): State<AppConfig>,
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> Result<Markup, HandlerError> {
    let user_id = current_user.require_authenticated()?;

    let recent_orders =
        queries::order::get_orders_for_user(user_id, constants::dashboard::RECENT_ORDERS_LIMIT)
            .await?;

    Ok(pages::dashboard(
        &current_user,
        flash.as_ref(),
        config.site_name(),
        recent_orders,
    ))
}
