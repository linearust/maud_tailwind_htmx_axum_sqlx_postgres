//! Auth service layer — decouples middleware from data queries.

use crate::{data::queries, models::UserId};

use super::CurrentUser;

/// Loads user context from database. Called by session_context middleware.
pub async fn load_user_context(user_id: &UserId) -> Result<Option<CurrentUser>, crate::data::errors::DataError> {
    match queries::user::get_user_info(user_id).await? {
        Some(info) => Ok(Some(CurrentUser::Authenticated {
            user_id: user_id.clone(),
            email: info.email,
            is_admin: info.is_admin,
        })),
        None => Ok(None),
    }
}
