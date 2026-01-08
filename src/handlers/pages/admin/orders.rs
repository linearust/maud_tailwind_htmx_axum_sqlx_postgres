use axum::{Extension, extract::{Query, State}};
use maud::Markup;
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    auth::CurrentUser,
    config::AppConfig,
    constants::admin::ITEMS_PER_PAGE,
    data::queries::admin,
    session::FlashMessage,
    handlers::errors::HandlerError,
    models::pagination::default_page,
    models::{admin::PaginatedResult, order::PaymentStatus},
    views::pages::admin as admin_views,
};

#[derive(Deserialize)]
pub struct OrdersQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    pub status: Option<PaymentStatus>,
}

pub async fn get_admin_orders(
    State(config): State<AppConfig>,
    State(db): State<PgPool>,
    Query(query): Query<OrdersQuery>,
    Extension(current_user): Extension<CurrentUser>,
    Extension(flash): Extension<Option<FlashMessage>>,
) -> Result<Markup, HandlerError> {
    let page = query.page.max(1);
    let status_filter = query.status;

    let orders = admin::get_orders_paginated(&db, status_filter, page, ITEMS_PER_PAGE).await?;

    let total_count = admin::get_total_order_count(&db, status_filter).await?;

    let paginated = PaginatedResult::new(orders, total_count, page, ITEMS_PER_PAGE);

    Ok(admin_views::orders(
        &current_user,
        flash.as_ref(),
        config.site_name(),
        paginated,
        status_filter,
    ))
}
