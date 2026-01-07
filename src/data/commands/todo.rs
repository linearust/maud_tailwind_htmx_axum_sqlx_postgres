use crate::{
    constants::errors,
    data::{ensure_rows_affected, errors::DataError},
    models::{todo::Todo, TodoId, UserId},
};
use sqlx::PgPool;

pub async fn create_todo(db: &PgPool, user_id: UserId, task: &str) -> Result<(), DataError> {
    sqlx::query!(
        "INSERT INTO todos(task, author_id) VALUES($1, $2)",
        task,
        user_id.as_i32()
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn toggle_todo_completion(
    db: &PgPool,
    user_id: UserId,
    todo_id: TodoId,
) -> Result<Todo, DataError> {
    let todo = sqlx::query_as!(
        Todo,
        "UPDATE todos SET is_done = NOT is_done
         WHERE todo_id = $1 AND author_id = $2
         RETURNING todo_id, task, is_done",
        todo_id.as_i32(),
        user_id.as_i32()
    )
    .fetch_one(db)
    .await
    .map_err(|e| crate::data::map_row_not_found(e, errors::TODO_NOT_FOUND))?;

    Ok(todo)
}

pub async fn delete_todo(db: &PgPool, user_id: UserId, todo_id: TodoId) -> Result<(), DataError> {
    let result = sqlx::query!(
        "DELETE FROM todos WHERE todo_id = $1 AND author_id = $2",
        todo_id.as_i32(),
        user_id.as_i32()
    )
    .execute(db)
    .await?;

    ensure_rows_affected(result, errors::TODO_NOT_FOUND)
}
