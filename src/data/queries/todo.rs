use sqlx::PgPool;

use crate::{data::errors::DataError, models::{todo::Todo, UserId}};

pub async fn get_todos_for_user(db: &PgPool, user_id: UserId) -> Result<Vec<Todo>, DataError> {
    let todos = sqlx::query_as!(
        Todo,
        "SELECT todo_id, task, is_done FROM todos WHERE author_id = $1 ORDER BY created_at DESC",
        user_id.as_i32()
    )
    .fetch_all(db)
    .await?;

    Ok(todos)
}
