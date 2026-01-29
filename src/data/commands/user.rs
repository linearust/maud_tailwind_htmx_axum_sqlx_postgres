use serde::{Deserialize, Serialize};

use crate::{data::errors::DataError, db::DB, models::UserId};

#[derive(Serialize)]
struct UserData {
    email: String,
}

#[derive(Deserialize)]
struct UserRecord {
    id: UserId,
}

pub async fn get_or_create_user(email: &str) -> Result<UserId, DataError> {
    // Check if user exists
    let mut result = DB
        .query("SELECT id FROM user WHERE email = $email LIMIT 1")
        .bind(("email", email.to_string()))
        .await?;

    let existing: Option<UserRecord> = result.take(0)?;

    if let Some(user) = existing {
        return Ok(user.id);
    }

    // Create new user
    let created: Option<UserRecord> = DB
        .create("user")
        .content(UserData { email: email.to_string() })
        .await?;

    Ok(created.expect("User should be created").id)
}
