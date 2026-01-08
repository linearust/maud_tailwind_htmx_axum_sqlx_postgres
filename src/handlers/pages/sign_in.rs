use axum::{Extension, extract::State};
use maud::Markup;

use crate::{
    auth::CurrentUser,
    config::AppConfig,
    handlers::errors::HandlerError,
    session::FlashMessage,
    views::pages,
};

pub async fn get_sign_in(
    State(config): State<AppConfig>,
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> Result<Markup, HandlerError> {
    Ok(pages::sign_in(&current_user, flash.as_ref(), config.site_name(), None, None))
}
