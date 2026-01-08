use sqlx::PgPool;

use crate::data::errors::DataError;

pub async fn get_or_create_user(db: &PgPool, email: &str) -> Result<i32, DataError> {
    let existing = sqlx::query!("SELECT user_id FROM users WHERE email = $1", email)
        .fetch_optional(db)
        .await?;

    if let Some(row) = existing {
        return Ok(row.user_id);
    }

    let row = sqlx::query!(
        "INSERT INTO users(email) VALUES($1) RETURNING user_id",
        email
    )
    .fetch_one(db)
    .await?;

    Ok(row.user_id)
}
