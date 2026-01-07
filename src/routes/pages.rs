use crate::{config::AppState, handlers::pages, paths};
use axum::{Router, routing::get};

pub fn public_page_routes() -> Router<AppState> {
    Router::new()
        .route(paths::pages::ROOT, get(pages::get_root))
        .route(paths::pages::SIGN_IN, get(pages::get_sign_in))
}

pub fn protected_page_routes() -> Router<AppState> {
    Router::new()
        .route(paths::pages::DASHBOARD, get(pages::get_dashboard))
        .route(paths::pages::TODOS, get(pages::get_todos))
        .route(paths::pages::TEXT_ANALYZER, get(pages::get_text_analyzer))
        .route(paths::pages::QUOTE, get(pages::get_quote))
        .route(paths::pages::CHECKOUT, get(pages::get_checkout))
        .route(paths::pages::PAYMENT_CONFIRMATION, get(pages::get_payment_confirmation))
}
