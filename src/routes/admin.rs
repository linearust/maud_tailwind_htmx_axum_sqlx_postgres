use crate::{config::AppState, handlers, middlewares, paths};
use axum::{middleware, Router, routing::{delete, get, post}};

pub fn admin_routes() -> Router<AppState> {
    Router::new()
        // Admin pages
        .route(paths::pages::admin::HOME, get(handlers::pages::admin::get_admin_home))
        .route(paths::pages::admin::USERS, get(handlers::pages::admin::get_admin_users))
        .route(paths::pages::admin::USER_DETAIL, get(handlers::pages::admin::get_admin_user_detail))
        .route(paths::pages::admin::ORDERS, get(handlers::pages::admin::get_admin_orders))
        .route(paths::pages::admin::ORDER_DETAIL, get(handlers::pages::admin::get_admin_order_detail))
        // Admin forms
        .route(paths::forms::admin::GRANT_ROLE, post(handlers::forms::admin::post_forms_admin_users_user_id_grant_role))
        // Admin actions
        .route(paths::actions::admin::REVOKE_ROLE, delete(handlers::actions::admin::delete_actions_admin_users_user_id_revoke_role))
        // Require admin middleware for all routes
        .layer(middleware::from_fn(middlewares::require_admin))
}
