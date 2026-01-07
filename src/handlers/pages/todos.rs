use axum::{Extension, extract::State};
use maud::Markup;
use sqlx::PgPool;

use crate::{
    auth::CurrentUser,
    config::AppConfig,
    data::queries,
    flash::FlashMessage,
    handlers::errors::HandlerError,
    views::pages,
};

pub async fn get_todos(
    State(config): State<AppConfig>,
    State(db): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> Result<Markup, HandlerError> {
    let user_id = current_user.require_authenticated();

    let todos = queries::todo::get_todos_for_user(&db, user_id).await?;

    Ok(pages::todos(&current_user, flash.as_ref(), config.site_name(), todos, None, None))
}
