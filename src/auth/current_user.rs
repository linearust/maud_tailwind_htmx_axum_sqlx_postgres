use crate::{constants::errors, data::errors::DataError, models::UserId};

pub const SESSION_USER_ID_KEY: &str = "authenticated_user_id";

/// Injected via Extension by session_context middleware.
///
/// Protected routes use `require_authentication` middleware which redirects
/// guests before handlers run. If `require_authenticated()` returns Err:
/// check route is in protected_routes(), verify middleware ordering
/// (session_layer → session_context → require_authentication).
#[derive(Clone, Debug)]
pub enum CurrentUser {
    Authenticated {
        user_id: UserId,
        email: String,
        is_admin: bool,
    },
    Guest,
}

impl CurrentUser {
    /// Only call in protected routes — returns error on Guest.
    /// If this returns Err, check route is in protected_routes() and
    /// verify middleware ordering (session_layer → session_context → require_authentication).
    pub fn require_authenticated(&self) -> Result<&UserId, DataError> {
        match self {
            CurrentUser::Authenticated { user_id, .. } => Ok(user_id),
            CurrentUser::Guest => Err(DataError::Unauthorized(errors::AUTHENTICATION_REQUIRED)),
        }
    }

    pub fn is_admin(&self) -> bool {
        match self {
            CurrentUser::Authenticated { is_admin, .. } => *is_admin,
            CurrentUser::Guest => false,
        }
    }

    pub fn is_authenticated(&self) -> bool {
        matches!(self, CurrentUser::Authenticated { .. })
    }
}
