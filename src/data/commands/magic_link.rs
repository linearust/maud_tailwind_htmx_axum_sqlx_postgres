use sqlx::PgPool;
use time::{Duration, OffsetDateTime};

use crate::constants::{auth::MAGIC_LINK_EXPIRY_MINUTES, messages};
use crate::data::{errors::DataError, map_row_unauthorized};

pub async fn create_magic_link(
    db: &PgPool,
    email: &str,
    token: &str,
) -> Result<(), DataError> {
    let expires_at = OffsetDateTime::now_utc() + Duration::minutes(MAGIC_LINK_EXPIRY_MINUTES);

    sqlx::query!("DELETE FROM magic_links WHERE email = $1", email)
        .execute(db)
        .await?;

    sqlx::query!(
        "INSERT INTO magic_links(token, email, expires_at) VALUES($1, $2, $3)",
        token,
        email,
        expires_at
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn verify_and_consume_magic_link(
    db: &PgPool,
    token: &str,
) -> Result<String, DataError> {
    let now = OffsetDateTime::now_utc();

    let row = sqlx::query!(
        "DELETE FROM magic_links
         WHERE token = $1 AND expires_at > $2
         RETURNING email",
        token,
        now
    )
    .fetch_one(db)
    .await
    .map_err(|e| map_row_unauthorized(e, messages::MAGIC_LINK_INVALID))?;

    Ok(row.email)
}
