use serde::Serialize;

use crate::{
    constants::errors,
    data::errors::DataError,
    db::DB,
    models::{todo::Todo, TodoId, UserId},
};

#[derive(Serialize)]
struct TodoData {
    task: String,
    author: surrealdb::RecordId,
}

pub async fn create_todo(user_id: &UserId, task: &str) -> Result<(), DataError> {
    let _: Option<Todo> = DB
        .create("todo")
        .content(TodoData {
            task: task.to_string(),
            author: user_id.clone().into_record_id(),
        })
        .await?;

    Ok(())
}

pub async fn toggle_todo_completion(user_id: &UserId, todo_id: &TodoId) -> Result<Todo, DataError> {
    let mut result = DB
        .query("UPDATE $todo SET is_done = !is_done WHERE author = $author RETURN id, task, is_done")
        .bind(("todo", todo_id.clone().into_record_id()))
        .bind(("author", user_id.clone().into_record_id()))
        .await?;

    let todo: Option<Todo> = result.take(0)?;
    todo.ok_or(DataError::NotFound(errors::TODO_NOT_FOUND))
}

pub async fn delete_todo(user_id: &UserId, todo_id: &TodoId) -> Result<(), DataError> {
    let mut result = DB
        .query("DELETE $todo WHERE author = $author RETURN BEFORE")
        .bind(("todo", todo_id.clone().into_record_id()))
        .bind(("author", user_id.clone().into_record_id()))
        .await?;

    let deleted: Option<Todo> = result.take(0)?;

    if deleted.is_none() {
        return Err(DataError::NotFound(errors::TODO_NOT_FOUND));
    }

    Ok(())
}
