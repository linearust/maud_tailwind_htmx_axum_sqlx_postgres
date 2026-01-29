use serde::Deserialize;

use crate::{data::errors::DataError, db::DB, models::UserId};

pub struct UserInfo {
    pub email: String,
    pub is_admin: bool,
}

#[derive(Deserialize)]
struct UserRow {
    email: String,
}

#[derive(Deserialize)]
struct AdminCheck {
    count: i64,
}

pub async fn get_user_info(user_id: &UserId) -> Result<Option<UserInfo>, DataError> {
    let user: Option<UserRow> = DB.select(user_id.clone().into_record_id()).await?;

    let Some(user) = user else {
        return Ok(None);
    };

    let mut result = DB
        .query("SELECT count() as count FROM user_role WHERE user = $user AND role = 'admin' GROUP ALL")
        .bind(("user", user_id.clone().into_record_id()))
        .await?;

    let admin_check: Option<AdminCheck> = result.take(0)?;
    let is_admin = admin_check.is_some_and(|c| c.count > 0);

    Ok(Some(UserInfo {
        email: user.email,
        is_admin,
    }))
}

pub async fn get_user_email(user_id: &UserId) -> Result<Option<String>, DataError> {
    let user: Option<UserRow> = DB.select(user_id.clone().into_record_id()).await?;
    Ok(user.map(|u| u.email))
}
