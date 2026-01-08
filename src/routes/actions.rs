use axum::{Router, routing::{delete, get, patch, post}};

use crate::{config::AppState, handlers::actions, paths::actions::relative};

pub fn public_action_routes() -> Router<AppState> {
    Router::new()
        .route(relative::VERIFY_MAGIC_LINK, get(actions::get_actions_auth_verify))
}

pub fn protected_action_routes() -> Router<AppState> {
    Router::new()
        .route(relative::SIGN_OUT, post(actions::post_actions_sign_out))
        .route(relative::TODOS_TODO_ID, delete(actions::delete_actions_todos_todo_id))
        .route(relative::TODOS_TODO_ID_TOGGLE, patch(actions::patch_actions_todos_todo_id_toggle))
        .route(relative::PAYMENT_INITIATE, post(actions::post_actions_payment_initiate))
        .route(relative::PAYMENT_VERIFY, get(actions::get_actions_payment_verify))
}
