use crate::{auth::CurrentUser, session::FlashMessage, views::helpers::format_price, models::order::Order, paths, views::layout::base::base_layout};
use maud::{Markup, html};

pub fn quote(
    current_user: &CurrentUser,
    flash: Option<&FlashMessage>,
    site_name: &str,
    order: &Order,
) -> Markup {
    let content = html! {
        div class="max-w-lg mx-auto" {
            h1 class="text-xl mb-3" { "Quote" }

            div class="space-y-3" {
                div class="space-y-1 text-sm" {
                    div class="flex justify-between" {
                        span class="text-gray-600" { "File" }
                        span { (order.filename) }
                    }
                    div class="flex justify-between" {
                        span class="text-gray-600" { "Size" }
                        span {
                            @let bytes = order.file_size as f64;
                            @if bytes < 1024.0 {
                                (format!("{} B", bytes))
                            } @else if bytes < 1024.0 * 1024.0 {
                                (format!("{:.2} KB", bytes / 1024.0))
                            } @else {
                                (format!("{:.2} MB", bytes / (1024.0 * 1024.0)))
                            }
                        }
                    }
                    div class="flex justify-between" {
                        span class="text-gray-600" { "Characters" }
                        span { (order.text_length.to_string()) }
                    }
                }

                div class="border-t pt-3" {
                    div class="flex justify-between items-center" {
                        span { "Total" }
                        span class="text-xl text-indigo-600" { "₩" (format_price(order.price_amount)) }
                    }
                }

                form method="post" action=(paths::actions::PAYMENT_INITIATE) {
                    input type="hidden" name="order_id" value=(order.id.to_string());
                    button
                        type="submit"
                        class="w-full bg-indigo-600 text-white px-3 py-2 hover:bg-indigo-700"
                        { "Pay Now" }
                }
            }
        }
    };

    base_layout(current_user, flash, site_name, "Quote", "Review your quote", content)
}
