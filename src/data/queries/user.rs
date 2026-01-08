use sqlx::PgPool;

use crate::{data::errors::DataError, models::UserId};

pub struct UserInfo {
    pub email: String,
    pub is_admin: bool,
}

pub async fn get_user_info(db: &PgPool, user_id: UserId) -> Result<Option<UserInfo>, DataError> {
    let result = sqlx::query!(
        r#"
        SELECT
            u.email,
            EXISTS(SELECT 1 FROM user_roles ur WHERE ur.user_id = u.user_id AND ur.role = 'admin') as "is_admin!"
        FROM users u
        WHERE u.user_id = $1
        "#,
        user_id.as_i32()
    )
    .fetch_optional(db)
    .await?;

    Ok(result.map(|row| UserInfo {
        email: row.email,
        is_admin: row.is_admin,
    }))
}

pub async fn get_user_email(db: &PgPool, user_id: UserId) -> Result<Option<String>, DataError> {
    let result = sqlx::query!("SELECT email FROM users WHERE user_id = $1", user_id.as_i32())
        .fetch_optional(db)
        .await?;

    Ok(result.map(|row| row.email))
}
