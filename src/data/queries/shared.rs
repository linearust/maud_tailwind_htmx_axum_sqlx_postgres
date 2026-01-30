use serde::Deserialize;
use surrealdb::RecordId;

use crate::{data::errors::DataError, db::DB, models::Role};

#[derive(Deserialize)]
struct CountResult {
    count: i64,
}

pub async fn check_user_is_admin(user_record_id: &RecordId) -> Result<bool, DataError> {
    let mut result = DB
        .query("SELECT count() as count FROM user_role WHERE user = $user AND role = $role GROUP ALL")
        .bind(("user", user_record_id.clone()))
        .bind(("role", Role::Admin.as_str()))
        .await?;
    let admin_check: Option<CountResult> = result.take(0)?;
    Ok(admin_check.is_some_and(|c| c.count > 0))
}
