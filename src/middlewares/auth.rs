use axum::{
    extract::Request,
    http::{HeaderValue, header},
    middleware::Next,
    response::{IntoResponse, Redirect},
};
use tower_sessions::Session;

use crate::{auth::CurrentUser, constants::messages, session::FlashMessage, paths};

pub async fn require_authentication(req: Request, next: Next) -> axum::response::Response {
    match req.extensions().get::<CurrentUser>() {
        Some(CurrentUser::Authenticated { .. }) => {
            let mut res = next.run(req).await;
            res.headers_mut().insert(
                header::CACHE_CONTROL,
                HeaderValue::from_static("no-store, no-cache, must-revalidate, private"),
            );
            res
        }
        _ => {
            let session = req.extensions().get::<Session>().cloned();
            if let Some(session) = session
                && let Err(e) = FlashMessage::error(messages::SIGN_IN_REQUIRED)
                    .set(&session)
                    .await
            {
                tracing::warn!("Failed to set flash message in auth middleware: {}", e);
            }
            Redirect::to(paths::pages::SIGN_IN).into_response()
        }
    }
}
