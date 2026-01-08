use std::collections::HashMap;

use axum::{Extension, Form, extract::State, http::StatusCode, response::{IntoResponse, Response}};
use sqlx::PgPool;
use tower_sessions::Session;
use validator::Validate;

use crate::{
    auth::CurrentUser,
    config::AppConfig,
    constants::messages,
    data::queries,
    email,
    session::FlashMessage,
    handlers::errors::HandlerResult,
    models::contact::{ContactForm, FIELD_EMAIL, FIELD_MESSAGE},
    paths,
    views::pages,
};

use super::parse_validation_errors;

pub async fn post_forms_contact(
    State(config): State<AppConfig>,
    State(db): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    session: Session,
    Form(form): Form<ContactForm>,
) -> HandlerResult {
    // Validate message for all users (email validated only for guests)
    if form.message.trim().is_empty() {
        let mut errors = HashMap::new();
        errors.insert(FIELD_MESSAGE.to_string(), "Message cannot be empty".to_string());
        let user_email = match &current_user {
            CurrentUser::Authenticated { email, .. } => Some(email.clone()),
            CurrentUser::Guest => None,
        };
        return Ok(render_validation_errors(&current_user, config.site_name(), &form, errors, user_email));
    }

    let email_to_use = match &current_user {
        CurrentUser::Authenticated { user_id, email, .. } => {
            queries::user::get_user_email(&db, *user_id)
                .await?
                .unwrap_or_else(|| email.clone())
        }
        CurrentUser::Guest => {
            if let Err(validation_errors) = form.validate() {
                let errors = parse_validation_errors(&validation_errors);
                return Ok(render_validation_errors(&current_user, config.site_name(), &form, errors, None));
            }
            form.email.clone()
        }
    };

    if let Err(e) = email::send_contact_inquiry(config.email(), &email_to_use, &form.message).await {
        tracing::error!("Failed to send contact inquiry email: {}", e);
        return Ok(FlashMessage::error(messages::EMAIL_SEND_FAILED)
            .set_and_redirect(&session, paths::pages::ROOT)
            .await?);
    }

    Ok(FlashMessage::success(messages::CONTACT_SENT)
        .set_and_redirect(&session, paths::pages::ROOT)
        .await?)
}

fn render_validation_errors(
    current_user: &CurrentUser,
    site_name: &str,
    form: &ContactForm,
    errors: HashMap<String, String>,
    user_email: Option<String>,
) -> Response {
    let email_to_show = user_email.as_deref().or(Some(&form.email));
    (
        StatusCode::BAD_REQUEST,
        pages::root(
            current_user,
            None,
            site_name,
            email_to_show,
            Some(&form.message),
            errors.get(FIELD_EMAIL).map(String::as_str),
            errors.get(FIELD_MESSAGE).map(String::as_str),
        ),
    )
        .into_response()
}
