use serde::Deserialize;
use surrealdb::RecordId;

use crate::{data::errors::DataError, db::DB, models::Role};

/// Query result type for COUNT() aggregations.
/// Use with: SELECT count() as count FROM ... GROUP ALL
#[derive(Deserialize)]
pub struct CountResult {
    pub count: i64,
}

impl CountResult {
    pub fn unwrap_or_zero(result: Option<Self>) -> i64 {
        result.map(|c| c.count).unwrap_or(0)
    }
}

/// Query result type for SUM() aggregations.
/// Use with: SELECT math::sum(field) as total FROM ... GROUP ALL
#[derive(Deserialize)]
pub struct SumResult {
    pub total: Option<i64>,
}

impl SumResult {
    pub fn unwrap_or_zero(result: Option<Self>) -> i64 {
        result.and_then(|s| s.total).unwrap_or(0)
    }
}

pub async fn check_user_is_admin(user_record_id: &RecordId) -> Result<bool, DataError> {
    let mut result = DB
        .query("SELECT count() as count FROM user_role WHERE user = $user AND role = $role GROUP ALL")
        .bind(("user", user_record_id.clone()))
        .bind(("role", Role::Admin.as_str()))
        .await?;
    let admin_check: Option<CountResult> = result.take(0)?;
    Ok(CountResult::unwrap_or_zero(admin_check) > 0)
}
