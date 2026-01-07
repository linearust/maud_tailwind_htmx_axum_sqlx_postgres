use crate::{
    auth::CurrentUser,
    flash::FlashMessage,
    models::admin::{OrderListItem, PaginatedResult},
    models::order::PaymentStatus,
    paths,
    views::{components::admin::{order_row, pagination}, layout::base::base_layout},
};
use maud::{html, Markup};

pub fn orders(
    current_user: &CurrentUser,
    flash: Option<&FlashMessage>,
    site_name: &str,
    paginated: PaginatedResult<OrderListItem>,
    filter: Option<PaymentStatus>,
) -> Markup {
    let content = html! {
        div class="max-w-6xl mx-auto" {
            h1 class="text-xl mb-6" { "Orders" }

            div class="flex gap-4 mb-4 text-sm" {
                (filter_tab("All", paths::pages::admin::ORDERS, filter.is_none()))
                (filter_tab("Paid", &paths::with_query_param(paths::pages::admin::ORDERS, "status", "paid"), matches!(filter, Some(PaymentStatus::Paid))))
                (filter_tab("Pending", &paths::with_query_param(paths::pages::admin::ORDERS, "status", "pending"), matches!(filter, Some(PaymentStatus::Pending))))
                (filter_tab("Failed", &paths::with_query_param(paths::pages::admin::ORDERS, "status", "failed"), matches!(filter, Some(PaymentStatus::Failed))))
            }

            @if paginated.items.is_empty() {
                p class="text-gray-500 py-4" { "No orders found" }
            } @else {
                table class="w-full text-sm" {
                    thead class="border-b" {
                        tr {
                            th class="text-left py-2 px-2" { "Order #" }
                            th class="text-left py-2 px-2" { "User" }
                            th class="text-right py-2 px-2" { "Amount" }
                            th class="text-center py-2 px-2" { "Status" }
                            th class="text-center py-2 px-2" { "Date" }
                            th class="text-center py-2 px-2" { "Actions" }
                        }
                    }
                    tbody {
                        @for order in &paginated.items {
                            (order_row(order, true))
                        }
                    }
                }

                (pagination(
                    &filter_path(filter),
                    paginated.page,
                    paginated.total_pages,
                    paginated.has_prev(),
                    paginated.has_next(),
                ))
            }
        }
    };

    base_layout(current_user, flash, site_name, "Orders", "Browse all orders", content)
}

fn filter_tab(label: &str, href: &str, is_active: bool) -> Markup {
    if is_active {
        html! {
            span class="border-b-2 border-indigo-600 pb-1" { (label) }
        }
    } else {
        html! {
            a href=(href) class="text-indigo-600 hover:underline pb-1" { (label) }
        }
    }
}

fn filter_path(filter: Option<PaymentStatus>) -> String {
    match filter {
        Some(status) => paths::with_query_param(paths::pages::admin::ORDERS, "status", status.as_str()),
        None => paths::pages::admin::ORDERS.to_string(),
    }
}

