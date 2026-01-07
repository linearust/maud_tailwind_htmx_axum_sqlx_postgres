use crate::models::UserId;

pub const SESSION_USER_ID_KEY: &str = "authenticated_user_id";

/// Represents the current user's authentication state.
///
/// This enum is injected into request handlers via `Extension<CurrentUser>` by the
/// `session_context` middleware. It tracks whether the current request is from an
/// authenticated user or a guest.
///
/// # Safety Considerations
///
/// The `require_authenticated()` method uses `unreachable!()` to enforce that it's
/// only called in protected routes. This is safe because:
///
/// 1. Protected routes are wrapped with the `require_authentication` middleware
///    (see `routes/mod.rs:protected_routes()`)
/// 2. This middleware ensures only `Authenticated` users can reach the handler
/// 3. Any guest attempting to access a protected route is redirected before the handler runs
///
/// ## Critical: Middleware Ordering
///
/// Layers are applied bottom-to-top but execute top-to-bottom:
/// 1. `session_layer` provides Session extractor
/// 2. `session_context` loads CurrentUser from session
/// 3. `require_authentication` is applied per-route group in protected_routes()
///
/// If middleware ordering changes, the `unreachable!()` assumption breaks and will panic.
#[derive(Clone, Debug)]
pub enum CurrentUser {
    /// An authenticated user with a valid session
    Authenticated {
        user_id: UserId,
        email: String,
        is_admin: bool,
    },
    /// A guest user (not authenticated)
    Guest,
}

impl CurrentUser {
    /// Extracts the user ID from an authenticated user.
    ///
    /// # Safety
    ///
    /// This method should ONLY be called in handlers within protected routes.
    /// Protected routes are defined in `routes/mod.rs:protected_routes()` and are
    /// guarded by the `require_authentication` middleware.
    ///
    /// # Panics
    ///
    /// This method will panic if called on a `Guest` user, which should be impossible
    /// in protected routes due to middleware enforcement. If you encounter this panic,
    /// check that:
    /// 1. The route is properly registered in `protected_routes()`
    /// 2. Middleware ordering hasn't changed
    /// 3. The handler is actually in a protected route group
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
}

