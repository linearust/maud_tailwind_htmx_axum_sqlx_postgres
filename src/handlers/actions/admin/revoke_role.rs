use axum::{Extension, extract::{Path, State}};
use sqlx::PgPool;
use tower_sessions::Session;

use crate::{
    auth::CurrentUser,
    constants::{errors, messages},
    data::commands::admin,
    flash::FlashMessage,
    handlers::errors::HandlerResult,
    models::UserId,
    paths::helpers,
};

pub async fn delete_actions_admin_users_user_id_revoke_role(
    State(db): State<PgPool>,
    Path(raw_user_id): Path<i32>,
    Extension(current_user): Extension<CurrentUser>,
    session: Session,
) -> HandlerResult {
    let admin_user_id = current_user.require_authenticated();
    let user_id = UserId::from_db(raw_user_id);

    if user_id == admin_user_id {
        return Ok(FlashMessage::error(errors::CANNOT_REVOKE_OWN_ADMIN)
            .set_and_redirect(&session, &helpers::user_detail_path(user_id))
            .await?);
    }

    admin::revoke_admin_role(&db, user_id).await?;

    Ok(FlashMessage::success(messages::ADMIN_ROLE_REVOKED)
        .set_and_redirect(&session, &helpers::user_detail_path(user_id))
        .await?)
}
