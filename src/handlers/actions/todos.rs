use axum::{Extension, extract::Path};

use crate::{
    auth::CurrentUser,
    data::commands,
    handlers::errors::HandlerResult,
    models::TodoId,
    views::{pages, response as htmx},
};

pub async fn delete_actions_todos_todo_id(
    Extension(current_user): Extension<CurrentUser>,
    Path(raw_todo_id): Path<String>,
) -> HandlerResult {
    let user_id = current_user.require_authenticated();
    let todo_id = TodoId::parse(&raw_todo_id).ok_or_else(|| {
        crate::data::errors::DataError::InvalidInput("Invalid todo ID".to_string())
    })?;

    commands::todo::delete_todo(user_id, &todo_id).await?;

    Ok(htmx::no_content_response())
}

pub async fn patch_actions_todos_todo_id_toggle(
    Extension(current_user): Extension<CurrentUser>,
    Path(raw_todo_id): Path<String>,
) -> HandlerResult {
    let user_id = current_user.require_authenticated();
    let todo_id = TodoId::parse(&raw_todo_id).ok_or_else(|| {
        crate::data::errors::DataError::InvalidInput("Invalid todo ID".to_string())
    })?;

    let todo = commands::todo::toggle_todo_completion(user_id, &todo_id).await?;
    Ok(htmx::html_fragment(pages::todo_item(&todo)))
}
