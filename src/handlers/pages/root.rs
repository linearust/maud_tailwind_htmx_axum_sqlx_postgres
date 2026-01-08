use axum::{Extension, extract::State};
use maud::Markup;
use sqlx::PgPool;

use crate::{
    auth::CurrentUser,
    config::AppConfig,
    data::queries,
    session::FlashMessage,
    handlers::errors::HandlerError,
    views::pages,
};

pub async fn get_root(
    State(config): State<AppConfig>,
    State(db): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> Result<Markup, HandlerError> {
    let user_email = match &current_user {
        CurrentUser::Authenticated { user_id, .. } => {
            queries::user::get_user_email(&db, *user_id).await?
        }
        CurrentUser::Guest => None,
    };

    Ok(pages::root(&current_user, flash.as_ref(), config.site_name(), user_email.as_deref(), None, None, None))
}
