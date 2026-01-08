use axum::{extract::{Request, State}, http::StatusCode, middleware::Next, response::IntoResponse};
use sqlx::PgPool;
use tower_sessions::Session;

use crate::{auth::{CurrentUser, SESSION_USER_ID_KEY}, data::queries, session::FlashMessage, models::UserId};

pub async fn session_context(
    State(db): State<PgPool>,
    session: Session,
    mut req: Request,
    next: Next,
) -> axum::response::Response {
    let current_user = match session.get::<i32>(SESSION_USER_ID_KEY).await {
        Ok(Some(user_id_param)) => {
            let user_id = UserId::from_db(user_id_param);
            match queries::user::get_user_info(&db, user_id).await {
                Ok(Some(info)) => CurrentUser::Authenticated {
                    user_id,
                    email: info.email,
                    is_admin: info.is_admin,
                },
                Ok(None) => {
                    tracing::warn!("User ID {} in session but not found in database", user_id_param);
                    CurrentUser::Guest
                }
                Err(e) => {
                    tracing::error!("Failed to fetch user info: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Session error").into_response();
                }
            }
        }
        Ok(None) => CurrentUser::Guest,
        Err(e) => {
            tracing::error!("Failed to read user_id from session: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Session error").into_response();
        }
    };

    let flash = match FlashMessage::get(&session).await {
        Ok(flash) => flash,
        Err(e) => {
            tracing::error!("Failed to read flash message from session: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Session error").into_response();
        }
    };

    req.extensions_mut().insert(current_user);
    req.extensions_mut().insert(flash);
    next.run(req).await
}
