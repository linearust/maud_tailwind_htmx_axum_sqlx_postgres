use axum::{Extension, extract::{Path, State}};
use sqlx::PgPool;

use crate::{
    auth::CurrentUser,
    data::commands,
    handlers::errors::HandlerResult,
    models::TodoId,
    views::{pages, response as htmx},
};

pub async fn delete_actions_todos_todo_id(
    State(db): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Path(todo_id_param): Path<i32>,
) -> HandlerResult {
    let user_id = current_user.require_authenticated();
    let todo_id = TodoId::from_db(todo_id_param);

    commands::todo::delete_todo(&db, user_id, todo_id).await?;

    Ok(htmx::empty_response())
}

pub async fn patch_actions_todos_todo_id_toggle(
    State(db): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Path(todo_id_param): Path<i32>,
) -> HandlerResult {
    let user_id = current_user.require_authenticated();
    let todo_id = TodoId::from_db(todo_id_param);

    let todo = commands::todo::toggle_todo_completion(&db, user_id, todo_id).await?;
    Ok(htmx::html_fragment(pages::todo_item(&todo)))
}
