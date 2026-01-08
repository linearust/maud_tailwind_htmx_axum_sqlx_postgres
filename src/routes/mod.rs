//! Routes by interaction type and protection level.

mod actions;
mod admin;
mod forms;
mod pages;

use axum::{Router, middleware};
use tower_http::services::ServeDir;
use tower_sessions::SessionManagerLayer;
use tower_sessions_sqlx_store::PostgresStore;

use crate::{config::AppState, handlers, middlewares, paths};

pub fn create_routes(state: AppState, session_layer: SessionManagerLayer<PostgresStore>) -> Router {
    Router::new()
        .nest_service(paths::static_files::BASE, ServeDir::new("static"))
        .merge(app_routes(state, session_layer))
        .layer(middleware::from_fn(middlewares::security_headers))
        .layer(middlewares::create_http_trace_layer())
}

fn app_routes(state: AppState, session_layer: SessionManagerLayer<PostgresStore>) -> Router {
    let state_clone = state.clone();

    Router::new()
        .merge(public_routes())
        .merge(protected_routes())
        .merge(admin_routes())
        .fallback(handlers::fallback::handle_404)
        .with_state(state)
        // CRITICAL: Layers apply bottom-to-top. session_layer → session_context → handler.
        // session_context loads CurrentUser from session — must run after session_layer.
        .layer(middleware::from_fn_with_state(state_clone, middlewares::session_context))
        .layer(session_layer)
}

fn admin_routes() -> Router<AppState> {
    admin::admin_routes()
        .layer(middleware::from_fn(middlewares::require_authentication))
}

fn public_routes() -> Router<AppState> {
    Router::new()
        .merge(pages::public_page_routes())
        .nest(paths::forms::BASE, forms::public_form_routes())
        .nest(paths::actions::BASE, actions::public_action_routes())
}

/// require_authentication redirects guests to sign-in.
/// MUST be outermost layer (runs first) and requires session_context to have loaded CurrentUser.
fn protected_routes() -> Router<AppState> {
    Router::new()
        .merge(pages::protected_page_routes())
        .nest(paths::forms::BASE, forms::protected_form_routes())
        .nest(paths::actions::BASE, actions::protected_action_routes())
        .layer(middleware::from_fn(middlewares::require_authentication))
}
