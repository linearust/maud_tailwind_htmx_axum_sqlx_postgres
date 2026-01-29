use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
use surrealdb::sql::Datetime;

use crate::constants::{auth::MAGIC_LINK_EXPIRY_MINUTES, messages};
use crate::data::errors::DataError;
use crate::db::DB;

#[derive(Serialize)]
struct MagicLinkData {
    email: String,
    expires_at: Datetime,
}

#[derive(Deserialize)]
struct MagicLinkRecord {
    email: String,
}

pub async fn create_magic_link(email: &str, token: &str) -> Result<(), DataError> {
    let expires_at = Datetime::from(Utc::now() + Duration::minutes(MAGIC_LINK_EXPIRY_MINUTES));

    // Delete existing magic links for this email
    DB.query("DELETE magic_link WHERE email = $email")
        .bind(("email", email.to_string()))
        .await?;

    // Create new magic link with token as ID
    let record_id = RecordId::from(("magic_link", token));
    let _: Option<MagicLinkRecord> = DB
        .create(record_id)
        .content(MagicLinkData {
            email: email.to_string(),
            expires_at,
        })
        .await?;

    Ok(())
}

pub async fn verify_and_consume_magic_link(token: &str) -> Result<String, DataError> {
    let record_id = RecordId::from(("magic_link", token));

    // Load the magic link
    let record: Option<MagicLinkRecord> = DB.select(&record_id).await?;

    let Some(link) = record else {
        return Err(DataError::Unauthorized(messages::MAGIC_LINK_INVALID));
    };

    // Delete it (consume)
    let _: Option<MagicLinkRecord> = DB.delete(&record_id).await?;

    Ok(link.email)
}
