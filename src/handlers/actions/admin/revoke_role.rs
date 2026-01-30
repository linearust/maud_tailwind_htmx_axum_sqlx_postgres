use axum::{Extension, extract::Path};
use tower_sessions::Session;

use crate::{
    auth::CurrentUser,
    constants::messages,
    data::commands::admin,
    session::FlashMessage,
    handlers::errors::HandlerResult,
    models::UserId,
    paths::helpers,
};

pub async fn delete_actions_admin_users_user_id_revoke_role(
    Path(raw_user_id): Path<String>,
    Extension(current_user): Extension<CurrentUser>,
    session: Session,
) -> HandlerResult {
    let admin_user_id = current_user.require_authenticated()?;
    let user_id = UserId::parse_or_invalid(&raw_user_id)?;

    admin::revoke_admin_role(&user_id, admin_user_id).await?;

    Ok(FlashMessage::success(messages::ADMIN_ROLE_REVOKED)
        .set_and_redirect(&session, &helpers::user_detail_path(&user_id))
        .await?)
}
