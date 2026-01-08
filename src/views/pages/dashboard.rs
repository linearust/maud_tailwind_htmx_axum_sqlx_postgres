use crate::{
    auth::CurrentUser,
    session::FlashMessage,
    views::helpers as formatting,
    models::order::OrderSummary,
    paths,
    views::layout::base::base_layout,
};
use maud::{html, Markup};

pub fn dashboard(
    current_user: &CurrentUser,
    flash: Option<&FlashMessage>,
    site_name: &str,
    recent_orders: Vec<OrderSummary>,
) -> Markup {
    let content = html! {
        div class="max-w-4xl mx-auto" {
            h1 class="text-xl mb-3" { "Orders" }

            @if recent_orders.is_empty() {
                p class="text-gray-500 py-4" { "No orders yet" }
            } @else {
                div class="overflow-x-auto" {
                    table class="w-full text-sm" {
                        thead class="border-b" {
                            tr {
                                th class="text-left py-2 px-2" { "Order #" }
                                th class="text-right py-2 px-2" { "Price" }
                                th class="text-center py-2 px-2" { "Status" }
                                th class="text-center py-2 px-2" { "Date" }
                            }
                        }
                        tbody {
                            @for order in recent_orders {
                                (order_row(&order))
                            }
                        }
                    }
                }
            }
        }
    };

    base_layout(current_user, flash, site_name, "Orders", "Your order history", content)
}

fn order_row(order: &OrderSummary) -> Markup {
    let status_class = order.payment_status.css_class();
    let status_text = order.payment_status.display_text();
    let date_display = formatting::format_datetime(order.created_at);

    html! {
        tr class="border-b" {
            td class="py-2 px-2" {
                a href=(paths::helpers::quote_path(order.order_id))
                    class="text-indigo-600 hover:underline"
                {
                    (order.order_number)
                }
            }
            td class="py-2 px-2 text-right" { "₩" (order.price_amount) }
            td class="py-2 px-2 text-center" {
                span class={"px-2 py-1 text-xs " (status_class)} {
                    (status_text)
                }
            }
            td class="py-2 px-2 text-center text-gray-600" { (date_display) }
        }
    }
}
