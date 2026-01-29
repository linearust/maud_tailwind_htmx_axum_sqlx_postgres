use axum::extract::Query;
use serde::Deserialize;
use tower_sessions::Session;

use crate::{
    auth::SESSION_USER_ID_KEY,
    constants::messages,
    data::commands,
    session::FlashMessage,
    handlers::errors::HandlerResult,
    paths,
};

#[derive(Deserialize)]
pub struct VerifyQuery {
    token: String,
}

pub async fn get_actions_auth_verify(
    session: Session,
    Query(query): Query<VerifyQuery>,
) -> HandlerResult {
    let email = match commands::magic_link::verify_and_consume_magic_link(&query.token).await
    {
        Ok(email) => email,
        Err(e) => {
            tracing::warn!("Magic link verification failed: {}", e);
            return Ok(FlashMessage::error(messages::MAGIC_LINK_INVALID)
                .set_and_redirect(&session, paths::pages::SIGN_IN)
                .await?);
        }
    };

    let user_id = commands::user::get_or_create_user(&email).await?;

    session.flush().await?;
    session.insert(SESSION_USER_ID_KEY, user_id).await?;

    Ok(FlashMessage::success(messages::SIGNED_IN)
        .set_and_redirect(&session, paths::pages::ROOT)
        .await?)
}
