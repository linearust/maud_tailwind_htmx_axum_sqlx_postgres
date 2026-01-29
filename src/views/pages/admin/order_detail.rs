use crate::{
    auth::CurrentUser,
    session::FlashMessage,
    views::helpers as formatting,
    models::admin::OrderDetail,
    paths,
    views::layout::base::base_layout,
};
use maud::{html, Markup};

pub fn order_detail(
    current_user: &CurrentUser,
    flash: Option<&FlashMessage>,
    site_name: &str,
    order: OrderDetail,
) -> Markup {
    let content = html! {
        div class="max-w-6xl mx-auto" {
            div class="mb-4" {
                a href=(paths::pages::admin::ORDERS)
                    class="text-indigo-600 hover:underline text-sm"
                {
                    "← Back to Orders"
                }
            }

            h1 class="text-xl mb-6" { "Order Details" }

            div class="mb-8 border p-4" {
                h2 class="text-lg mb-3" { "Order Information" }
                div class="space-y-2 text-sm" {
                    div {
                        span class="text-gray-600" { "Order Number: " }
                        span { (order.order_number) }
                    }
                    div {
                        span class="text-gray-600" { "Order ID: " }
                        span class="font-mono text-xs" { (order.id) }
                    }
                    div {
                        span class="text-gray-600" { "Status: " }
                        span class={"px-2 py-1 text-xs " (order.payment_status.css_class())} {
                            (order.payment_status.display_text())
                        }
                    }
                    div {
                        span class="text-gray-600" { "Amount: " }
                        span { "₩" (formatting::format_price(order.price_amount)) }
                    }
                    div {
                        span class="text-gray-600" { "Created: " }
                        span { (formatting::format_datetime(order.created_at)) }
                    }
                    @if let Some(paid_at) = order.paid_at {
                        div {
                            span class="text-gray-600" { "Paid: " }
                            span { (formatting::format_datetime(paid_at)) }
                        }
                    }
                }
            }

            div class="mb-8 border p-4" {
                h2 class="text-lg mb-3" { "User Information" }
                div class="space-y-2 text-sm" {
                    div {
                        span class="text-gray-600" { "Email: " }
                        a href=(paths::with_param(paths::pages::admin::USER_DETAIL, "user_id", &order.user))
                            class="text-indigo-600 hover:underline"
                        {
                            (order.user_email)
                        }
                    }
                    div {
                        span class="text-gray-600" { "User ID: " }
                        span class="font-mono text-xs" { (order.user) }
                    }
                }
            }

            @if order.payment_key.is_some() {
                div class="mb-8 border p-4" {
                    h2 class="text-lg mb-3" { "Payment Information" }
                    div class="space-y-2 text-sm" {
                        @if let Some(payment_key) = &order.payment_key {
                            div {
                                span class="text-gray-600" { "Payment Key: " }
                                span class="font-mono text-xs" { (payment_key) }
                            }
                        }
                    }
                }
            }

            div class="border p-4" {
                h2 class="text-lg mb-3" { "File Information" }
                div class="space-y-2 text-sm" {
                    div {
                        span class="text-gray-600" { "Filename: " }
                        span { (order.filename) }
                    }
                    div {
                        span class="text-gray-600" { "Character Count: " }
                        span { (order.text_length) }
                    }
                    div {
                        span class="text-gray-600" { "Price Calculation: " }
                        span { (order.text_length) " characters × ₩1 = ₩" (formatting::format_price(order.price_amount)) }
                    }
                }
            }
        }
    };

    base_layout(
        current_user,
        flash,
        site_name,
        "Order Details",
        &format!("Details for order {}", order.order_number),
        content,
    )
}
