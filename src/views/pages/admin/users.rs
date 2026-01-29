use crate::{
    auth::CurrentUser,
    session::FlashMessage,
    views::helpers as formatting,
    models::admin::{PaginatedResult, UserListItem},
    paths,
    views::{components::admin::pagination, layout::base::base_layout},
};
use maud::{html, Markup};

pub fn users(
    current_user: &CurrentUser,
    flash: Option<&FlashMessage>,
    site_name: &str,
    paginated: PaginatedResult<UserListItem>,
) -> Markup {
    let content = html! {
        div class="max-w-6xl mx-auto" {
            h1 class="text-xl mb-6" { "Users" }

            @if paginated.items.is_empty() {
                p class="text-gray-500 py-4" { "No users found" }
            } @else {
                table class="w-full text-sm" {
                    thead class="border-b" {
                        tr {
                            th class="text-left py-2 px-2" { "Email" }
                            th class="text-left py-2 px-2" { "Role" }
                            th class="text-center py-2 px-2" { "Signup Date" }
                            th class="text-center py-2 px-2" { "Orders" }
                            th class="text-right py-2 px-2" { "Total Spent" }
                            th class="text-center py-2 px-2" { "Actions" }
                        }
                    }
                    tbody {
                        @for user in &paginated.items {
                            (user_row(user))
                        }
                    }
                }

                (pagination(
                    paths::pages::admin::USERS,
                    paginated.page,
                    paginated.total_pages,
                    paginated.has_prev(),
                    paginated.has_next(),
                ))
            }
        }
    };

    base_layout(current_user, flash, site_name, "Users", "Browse all users", content)
}

fn user_row(user: &UserListItem) -> Markup {
    let date_display = formatting::format_datetime(user.created_at);

    html! {
        tr class="border-b" {
            td class="py-2 px-2" { (user.email) }
            td class="py-2 px-2" {
                @if user.is_admin {
                    span class="px-2 py-1 text-xs bg-indigo-100 text-indigo-800" {
                        "Admin"
                    }
                }
            }
            td class="py-2 px-2 text-center text-gray-600" { (date_display) }
            td class="py-2 px-2 text-center" { (user.order_count) }
            td class="py-2 px-2 text-right" { "₩" (formatting::format_price(user.total_spent)) }
            td class="py-2 px-2 text-center" {
                a href=(paths::with_param(paths::pages::admin::USER_DETAIL, "user_id", &user.id))
                    class="text-indigo-600 hover:underline text-sm"
                {
                    "View"
                }
            }
        }
    }
}
