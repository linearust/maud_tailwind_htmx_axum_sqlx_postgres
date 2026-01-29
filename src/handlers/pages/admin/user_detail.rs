use axum::{Extension, extract::{Path, Query, State}};
use maud::Markup;

use crate::{
    auth::CurrentUser,
    config::AppConfig,
    constants::admin::ITEMS_PER_PAGE,
    data::queries::admin,
    session::FlashMessage,
    handlers::errors::HandlerError,
    models::{pagination::PaginationQuery, admin::PaginatedResult, UserId},
    views::pages::admin as admin_views,
};

pub async fn get_admin_user_detail(
    State(config): State<AppConfig>,
    Path(raw_user_id): Path<String>,
    Query(query): Query<PaginationQuery>,
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> Result<Markup, HandlerError> {
    let page = query.page.max(1);
    let user_id = UserId::parse(&raw_user_id).ok_or_else(|| {
        crate::data::errors::DataError::InvalidInput("Invalid user ID".to_string())
    })?;

    let user = admin::get_user_detail(&user_id).await?;

    let orders = admin::get_user_orders(&user_id, page, ITEMS_PER_PAGE).await?;

    let total_count = admin::get_user_order_count(&user_id).await?;
    let paginated_orders = PaginatedResult::new(orders, total_count, page, ITEMS_PER_PAGE);

    Ok(admin_views::user_detail(
        &current_user,
        flash.as_ref(),
        config.site_name(),
        user,
        paginated_orders,
    ))
}
