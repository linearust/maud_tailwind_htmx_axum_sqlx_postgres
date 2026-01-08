use crate::models::UserId;

pub const SESSION_USER_ID_KEY: &str = "authenticated_user_id";

/// Injected via Extension by session_context middleware.
///
/// # Safety: `require_authenticated()` uses `unreachable!()`
///
/// Safe because protected routes use `require_authentication` middleware which redirects
/// guests before handlers run. If this panics: check route is in protected_routes(),
/// verify middleware ordering (session_layer → session_context → require_authentication).
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
    /// Only call in protected routes — panics on Guest. See enum docs for safety.
    pub fn require_authenticated(&self) -> UserId {
        match self {
            CurrentUser::Authenticated { user_id, .. } => *user_id,
            CurrentUser::Guest => unreachable!(
                "Protected route accessed by guest user. This indicates a middleware \
                configuration error. Ensure the route is in protected_routes() and \
                middleware ordering is correct."
            ),
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
