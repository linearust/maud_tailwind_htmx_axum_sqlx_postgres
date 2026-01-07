use sqlx::PgPool;
use crate::{constants::admin::ROLE_ADMIN, data::errors::DataError, models::UserId};

pub async fn grant_admin_role(
    db: &PgPool,
    user_id: UserId,
    granted_by: UserId,
) -> Result<(), DataError> {
    sqlx::query!(
        r#"
        INSERT INTO user_roles (user_id, role, granted_by)
        VALUES ($1, $2, $3)
        ON CONFLICT (user_id, role) DO NOTHING
        "#,
        user_id.as_i32(),
        ROLE_ADMIN,
        granted_by.as_i32()
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn revoke_admin_role(db: &PgPool, user_id: UserId) -> Result<(), DataError> {
    sqlx::query!(
        r#"
        DELETE FROM user_roles
        WHERE user_id = $1 AND role = $2
        "#,
        user_id.as_i32(),
        ROLE_ADMIN
    )
    .execute(db)
    .await?;

    Ok(())
}
