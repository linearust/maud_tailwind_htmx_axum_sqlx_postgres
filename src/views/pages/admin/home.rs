use crate::{
    auth::CurrentUser,
    session::FlashMessage,
    views::helpers as formatting,
    models::admin::AdminStats,
    paths,
    views::{components::admin::stats_card, layout::base::base_layout},
};
use maud::{html, Markup};

pub fn home(
    current_user: &CurrentUser,
    flash: Option<&FlashMessage>,
    site_name: &str,
    stats: AdminStats,
) -> Markup {
    let content = html! {
        div class="max-w-6xl mx-auto" {
            h1 class="text-xl mb-6" { "Admin Dashboard" }

            div class="grid grid-cols-4 gap-4 mb-8" {
                (stats_card("Total Users", &stats.total_users.to_string()))
                (stats_card("Total Orders", &stats.total_orders.to_string()))
                (stats_card("Total Revenue", &format!("₩{}", formatting::format_price(stats.total_revenue))))
                (stats_card("Orders (7d)", &stats.orders_last_7_days.to_string()))
            }

            div class="space-y-2" {
                h2 class="text-lg mb-3" { "Quick Links" }
                div {
                    a href=(paths::pages::admin::USERS)
                        class="text-indigo-600 hover:underline"
                    {
                        "View All Users"
                    }
                }
                div {
                    a href=(paths::pages::admin::ORDERS)
                        class="text-indigo-600 hover:underline"
                    {
                        "View All Orders"
                    }
                }
            }
        }
    };

    base_layout(
        current_user,
        flash,
        site_name,
        "Admin Dashboard",
        "System overview and quick access",
        content,
    )
}
