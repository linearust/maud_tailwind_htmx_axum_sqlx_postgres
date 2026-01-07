use crate::{auth::CurrentUser, flash::FlashMessage, models::order::Order, paths, views::layout::base::base_layout};
use maud::{Markup, html};

pub fn payment_confirmation(
    current_user: &CurrentUser,
    flash: Option<&FlashMessage>,
    site_name: &str,
    order: &Order,
) -> Markup {
    let word_count = order.text_content.split_whitespace().count();

    let content = html! {
        div class="max-w-lg mx-auto" {
            p class="text-green-700 mb-3" { "✓ Payment successful" }

            h1 class="text-xl mb-3" { "Analysis Complete" }

            div class="space-y-3" {
                div class="grid grid-cols-2 gap-3 text-sm" {
                    div class="text-center py-3 border" {
                        p class="text-2xl" { (order.text_length) }
                        p class="text-gray-600 mt-1" { "Characters" }
                    }
                    div class="text-center py-3 border" {
                        p class="text-2xl" { (word_count) }
                        p class="text-gray-600 mt-1" { "Words" }
                    }
                }

                a
                    href=(paths::pages::TEXT_ANALYZER)
                    class="block w-full bg-indigo-600 text-white px-3 py-2 hover:bg-indigo-700 text-center"
                    { "Analyze Another File" }
            }
        }
    };

    base_layout(current_user, flash, site_name, "Payment Confirmation", "Text analysis results", content)
}
