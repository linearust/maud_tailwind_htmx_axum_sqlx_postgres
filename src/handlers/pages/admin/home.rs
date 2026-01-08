use axum::{Extension, extract::State};
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

pub async fn get_admin_home(
    State(config): State<AppConfig>,
    State(db): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> Result<Markup, HandlerError> {
    let stats = admin::get_admin_stats(&db).await?;

    Ok(admin_views::home(
        &current_user,
        flash.as_ref(),
        config.site_name(),
        stats,
    ))
}
