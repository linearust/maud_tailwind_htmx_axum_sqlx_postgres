use axum::{Extension, extract::{Path, State}};
use maud::Markup;
use sqlx::PgPool;

use crate::{
    auth::CurrentUser,
    config::AppConfig,
    data::queries::admin,
    session::FlashMessage,
    handlers::errors::HandlerError,
    views::pages::admin as admin_views,
};

pub async fn get_admin_order_detail(
    State(config): State<AppConfig>,
    State(db): State<PgPool>,
    Path(order_id): Path<String>,
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> Result<Markup, HandlerError> {
    let order = admin::get_order_detail(&db, &order_id).await?;

    Ok(admin_views::order_detail(
        &current_user,
        flash.as_ref(),
        config.site_name(),
        order,
    ))
}
