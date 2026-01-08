use axum::{Extension, extract::State, http::StatusCode};
use maud::Markup;

use crate::{auth::CurrentUser, config::AppConfig, session::FlashMessage, views::pages};

pub async fn handle_404(
    State(config): State<AppConfig>,
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> (StatusCode, Markup) {
    (StatusCode::NOT_FOUND, pages::not_found(&current_user, flash.as_ref(), config.site_name()))
}
