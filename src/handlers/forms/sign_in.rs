use axum::{Extension, Form, extract::State, http::StatusCode, response::{IntoResponse, Response}};
use tower_sessions::Session;
use validator::Validate;

use crate::{
    auth::{self, CurrentUser},
    config::AppConfig,
    constants::messages,
    data::commands,
    email,
    handlers::errors::HandlerResult,
    models::sign_in::{FIELD_EMAIL, MagicLinkRequestForm},
    paths,
    session::FlashMessage,
    views::pages,
};

use super::parse_validation_errors;

pub async fn post_forms_sign_in(
    State(config): State<AppConfig>,
    Extension(current_user): Extension<CurrentUser>,
    session: Session,
    Form(form): Form<MagicLinkRequestForm>,
) -> HandlerResult {
    if let Err(validation_errors) = form.validate() {
        return Ok(render_validation_errors(&current_user, config.site_name(), &form, &validation_errors));
    }

    let token = auth::generate_token();
    commands::magic_link::create_magic_link(&form.email, &token).await?;

    if let Err(e) = email::send_magic_link(config.email(), &form.email, &token).await {
        tracing::error!("Failed to send magic link email: {}", e);
        return Ok(FlashMessage::error(messages::EMAIL_SEND_FAILED)
            .set_and_redirect(&session, paths::pages::SIGN_IN)
            .await?);
    }

    Ok(FlashMessage::success(messages::MAGIC_LINK_SENT)
        .set_and_redirect(&session, paths::pages::SIGN_IN)
        .await?)
}

fn render_validation_errors(
    current_user: &CurrentUser,
    site_name: &str,
    form: &MagicLinkRequestForm,
    validation_errors: &validator::ValidationErrors,
) -> Response {
    let errors = parse_validation_errors(validation_errors);
    (
        StatusCode::BAD_REQUEST,
        pages::sign_in(
            current_user,
            None,
            site_name,
            Some(&form.email),
            errors.get(FIELD_EMAIL).map(String::as_str),
        ),
    )
        .into_response()
}
