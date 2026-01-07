use crate::{
    auth::CurrentUser,
    flash::FlashMessage,
    formatting,
    models::admin::{OrderListItem, PaginatedResult, UserDetail},
    paths,
    views::{components::admin::{order_row, pagination}, layout::base::base_layout},
};
use maud::{html, Markup};

pub fn user_detail(
    current_user: &CurrentUser,
    flash: Option<&FlashMessage>,
    site_name: &str,
    user: UserDetail,
    paginated_orders: PaginatedResult<OrderListItem>,
) -> Markup {
    let content = html! {
        div class="max-w-6xl mx-auto" {
            div class="mb-4" {
                a href=(paths::pages::admin::USERS)
                    class="text-indigo-600 hover:underline text-sm"
                {
                    "← Back to Users"
                }
            }

            h1 class="text-xl mb-6" { "User Details" }

            div class="mb-8 border p-4" {
                h2 class="text-lg mb-3" { "User Information" }
                div class="space-y-2 text-sm" {
                    div {
                        span class="text-gray-600" { "Email: " }
                        span { (user.email) }
                    }
                    div {
                        span class="text-gray-600" { "User ID: " }
                        span class="font-mono text-xs" { (user.user_id) }
                    }
                    div {
                        span class="text-gray-600" { "Signup Date: " }
                        span { (formatting::format_datetime(user.created_at)) }
                    }
                    div {
                        span class="text-gray-600" { "Total Orders: " }
                        span { (user.order_count) }
                    }
                    div {
                        span class="text-gray-600" { "Total Spent: " }
                        span { "₩" (formatting::format_price(user.total_spent)) }
                    }
                }
            }

            div class="mb-8 border p-4" {
                h2 class="text-lg mb-3" { "Admin Role" }
                @if user.is_admin {
                    div class="mb-3" {
                        span class="px-2 py-1 text-xs bg-indigo-100 text-indigo-800" {
                            "Admin"
                        }
                    }
                    form method="post"
                        action=(paths::with_param(paths::actions::admin::REVOKE_ROLE, "user_id", &user.user_id))
                        hx-delete=(paths::with_param(paths::actions::admin::REVOKE_ROLE, "user_id", &user.user_id))
                        hx-target="body"
                        hx-swap="outerHTML"
                    {
                        button type="submit"
                            class="text-sm text-red-600 hover:underline"
                        {
                            "Revoke Admin Role"
                        }
                    }
                } @else {
                    p class="text-sm text-gray-600 mb-3" { "This user is not an admin" }
                    form method="post"
                        action=(paths::with_param(paths::forms::admin::GRANT_ROLE, "user_id", &user.user_id))
                    {
                        button type="submit"
                            class="text-sm text-indigo-600 hover:underline"
                        {
                            "Grant Admin Role"
                        }
                    }
                }
            }

            div {
                h2 class="text-lg mb-3" { "Orders" }
                @if paginated_orders.items.is_empty() {
                    p class="text-gray-500 py-4" { "No orders yet" }
                } @else {
                    table class="w-full text-sm" {
                        thead class="border-b" {
                            tr {
                                th class="text-left py-2 px-2" { "Order #" }
                                th class="text-right py-2 px-2" { "Amount" }
                                th class="text-center py-2 px-2" { "Status" }
                                th class="text-center py-2 px-2" { "Date" }
                            }
                        }
                        tbody {
                            @for order in &paginated_orders.items {
                                (order_row(order, false))
                            }
                        }
                    }

                    (pagination(
                        &paths::with_param(paths::pages::admin::USER_DETAIL, "user_id", &user.user_id),
                        paginated_orders.page,
                        paginated_orders.total_pages,
                        paginated_orders.has_prev(),
                        paginated_orders.has_next(),
                    ))
                }
            }
        }
    };

    base_layout(
        current_user,
        flash,
        site_name,
        "User Details",
        &format!("Details for {}", user.email),
        content,
    )
}

