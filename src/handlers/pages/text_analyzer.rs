use axum::{Extension, extract::State};
use maud::Markup;

use crate::{
    auth::CurrentUser,
    config::AppConfig,
    session::FlashMessage,
    handlers::errors::HandlerError,
    views::pages,
};

pub async fn get_text_analyzer(
    State(config): State<AppConfig>,
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> Result<Markup, HandlerError> {
    Ok(pages::text_analyzer(&current_user, flash.as_ref(), config.site_name()))
}
