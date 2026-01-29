use serde::Serialize;

use crate::{data::errors::DataError, db::DB, models::{Role, UserId}};

#[derive(Serialize)]
struct UserRoleData {
    user: surrealdb::RecordId,
    role: String,
    granted_by: Option<surrealdb::RecordId>,
}

#[derive(serde::Deserialize)]
struct UserRoleRecord {
    #[allow(dead_code)]
    user: surrealdb::RecordId,
}

pub async fn grant_admin_role(user_id: &UserId, granted_by: &UserId) -> Result<(), DataError> {
    let _: Option<UserRoleRecord> = DB
        .create("user_role")
        .content(UserRoleData {
            user: user_id.clone().into_record_id(),
            role: Role::Admin.as_str().to_string(),
            granted_by: Some(granted_by.clone().into_record_id()),
        })
        .await?;

    Ok(())
}

pub async fn revoke_admin_role(user_id: &UserId) -> Result<(), DataError> {
    DB.query("DELETE user_role WHERE user = $user AND role = $role")
        .bind(("user", user_id.clone().into_record_id()))
        .bind(("role", Role::Admin.as_str().to_string()))
        .await?;

    Ok(())
}
