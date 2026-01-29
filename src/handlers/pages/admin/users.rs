use axum::{Extension, extract::{Query, State}};
use maud::Markup;

use crate::{
    auth::CurrentUser,
    config::AppConfig,
    constants::admin::ITEMS_PER_PAGE,
    data::queries::admin,
    session::FlashMessage,
    handlers::errors::HandlerError,
    models::pagination::PaginationQuery,
    models::admin::PaginatedResult,
    views::pages::admin as admin_views,
};

pub async fn get_admin_users(
    State(config): State<AppConfig>,
    Query(query): Query<PaginationQuery>,
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> Result<Markup, HandlerError> {
    let page = query.page.max(1);

    let users = admin::get_users_paginated(page, ITEMS_PER_PAGE).await?;

    let total_count = admin::get_total_user_count().await?;

    let paginated = PaginatedResult::new(users, total_count, page, ITEMS_PER_PAGE);

    Ok(admin_views::users(
        &current_user,
        flash.as_ref(),
        config.site_name(),
        paginated,
    ))
}
