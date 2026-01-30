use serde::Deserialize;

use crate::{data::errors::DataError, db::DB, models::UserId};

use super::shared::check_user_is_admin;

pub struct UserInfo {
    pub email: String,
    pub is_admin: bool,
}

#[derive(Deserialize)]
struct UserRow {
    email: String,
}

pub async fn get_user_info(user_id: &UserId) -> Result<Option<UserInfo>, DataError> {
    let user: Option<UserRow> = DB.select(user_id.clone().into_record_id()).await?;

    let Some(user) = user else {
        return Ok(None);
    };

    let is_admin = check_user_is_admin(&user_id.clone().into_record_id()).await?;

    Ok(Some(UserInfo {
        email: user.email,
        is_admin,
    }))
}

pub async fn get_user_email(user_id: &UserId) -> Result<Option<String>, DataError> {
    let user: Option<UserRow> = DB.select(user_id.clone().into_record_id()).await?;
    Ok(user.map(|u| u.email))
}
