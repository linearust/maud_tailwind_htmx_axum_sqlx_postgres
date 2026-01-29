use tower_sessions::SessionManagerLayer;

use crate::{constants, session_store::SurrealSessionStore};

pub fn init_session() -> SessionManagerLayer<SurrealSessionStore> {
    let session_store = SurrealSessionStore::new();

    SessionManagerLayer::new(session_store)
        .with_expiry(tower_sessions::Expiry::OnInactivity(time::Duration::days(
            constants::auth::SESSION_EXPIRY_DAYS,
        )))
        .with_same_site(tower_sessions::cookie::SameSite::Lax)
}
