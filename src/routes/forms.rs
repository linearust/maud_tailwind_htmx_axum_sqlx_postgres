use axum::{Router, routing::post};

use crate::{config::AppState, handlers::forms, paths::forms::relative};

pub fn public_form_routes() -> Router<AppState> {
    Router::new()
        .route(relative::SIGN_IN, post(forms::post_forms_sign_in))
        .route(relative::CONTACT, post(forms::post_forms_contact))
}

pub fn protected_form_routes() -> Router<AppState> {
    Router::new()
        .route(relative::TODOS, post(forms::post_forms_todos))
        .route(relative::TEXT_ANALYZER, post(forms::post_forms_text_analyzer))
}
