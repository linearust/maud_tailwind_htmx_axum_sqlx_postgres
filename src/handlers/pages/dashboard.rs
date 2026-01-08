use axum::{Extension, extract::State};
use sqlx::PgPool;

use crate::{
    auth::CurrentUser,
    config::AppConfig,
    data::queries,
    session::FlashMessage,
    handlers::errors::HandlerError,
    views::pages,
};
use maud::Markup;

pub async fn get_dashboard(
    State(config): State<AppConfig>,
    State(db): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> Result<Markup, HandlerError> {
    let user_id = current_user.require_authenticated();

    let recent_orders = queries::order::get_orders_for_user(&db, user_id, 10).await?;

    Ok(pages::dashboard(
        &current_user,
        flash.as_ref(),
        config.site_name(),
        recent_orders,
    ))
}
