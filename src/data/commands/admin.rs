use serde::{Deserialize, Serialize};

use crate::{constants::errors, data::{errors::DataError, queries::shared::check_user_is_admin}, db::DB, models::{Role, UserId}};

#[derive(Serialize)]
struct UserRoleData {
    user: surrealdb::RecordId,
    role: String,
    granted_by: Option<surrealdb::RecordId>,
}

#[derive(Deserialize)]
struct UserRoleRecord {
    #[allow(dead_code)]
    user: surrealdb::RecordId,
}

pub async fn grant_admin_role(user_id: &UserId, granted_by: &UserId) -> Result<(), DataError> {
    // Check if user already has admin role
    if check_user_is_admin(&user_id.clone().into_record_id()).await? {
        return Err(DataError::InvalidInput("User already has admin role".to_string()));
    }

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

pub async fn revoke_admin_role(user_id: &UserId, revoked_by: &UserId) -> Result<(), DataError> {
    if user_id == revoked_by {
        return Err(DataError::InvalidInput(errors::CANNOT_REVOKE_OWN_ADMIN.to_string()));
    }

    DB.query("DELETE user_role WHERE user = $user AND role = $role")
        .bind(("user", user_id.clone().into_record_id()))
        .bind(("role", Role::Admin.as_str().to_string()))
        .await?;

    Ok(())
}
