use axum::{Extension, extract::{Path, State}};
use sqlx::PgPool;
use tower_sessions::Session;

use crate::{
    auth::CurrentUser,
    constants::messages,
    data::commands::admin,
    flash::FlashMessage,
    handlers::errors::HandlerResult,
    paths::helpers,
};

pub async fn post_forms_admin_users_user_id_grant_role(
    State(db): State<PgPool>,
    Path(user_id): Path<i32>,
    Extension(current_user): Extension<CurrentUser>,
    session: Session,
) -> HandlerResult {
    let admin_user_id = current_user.require_authenticated();

    admin::grant_admin_role(&db, user_id, admin_user_id).await?;

    Ok(FlashMessage::success(messages::ADMIN_ROLE_GRANTED)
        .set_and_redirect(&session, &helpers::user_detail_path(user_id))
        .await?)
}
