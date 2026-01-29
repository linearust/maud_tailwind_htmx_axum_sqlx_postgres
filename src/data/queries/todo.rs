use crate::{
    data::errors::DataError,
    db::DB,
    models::{todo::Todo, UserId},
};

pub async fn get_todos_for_user(user_id: &UserId) -> Result<Vec<Todo>, DataError> {
    let mut result = DB
        .query("SELECT id, task, is_done, created_at FROM todo WHERE author = $author ORDER BY created_at DESC")
        .bind(("author", user_id.clone().into_record_id()))
        .await?;

    let todos: Vec<Todo> = result.take(0)?;
    Ok(todos)
}
