use crate::{auth::CurrentUser, constants::{cdn, payment}, flash::FlashMessage, formatting::format_price, models::order::Order, paths, views::layout::base};
use maud::{Markup, PreEscaped, html};

fn toss_payment_script(client_key: &str, order: &Order, success_url: &str, fail_url: &str) -> Markup {
    html! {
        script src=(cdn::TOSS_PAYMENTS_SDK_URL) {}
        script {
            (PreEscaped(format!(r#"
                const button = document.getElementById('payment-button');

                try {{
                    const tossPayments = TossPayments('{client_key}');
                    button.disabled = false;

                    const paymentParams = {{
                        amount: {amount},
                        orderId: '{order_id}',
                        orderName: '{prefix} - {filename}',
                        successUrl: window.location.origin + '{success_url}',
                        failUrl: window.location.origin + '{fail_url}'
                    }};

                    button.addEventListener('click', function() {{
                        console.log('Payment request parameters:', paymentParams);

                        tossPayments.requestPayment('카드', paymentParams)
                        .catch(function(error) {{
                            console.error('Payment request failed:', error);
                            alert('결제 요청 실패: ' + (error.message || error.code));
                        }});
                    }});
                }} catch (error) {{
                    console.error('Toss Payments initialization failed:', error);
                    button.disabled = true;
                    button.textContent = 'Payment Error';
                }}
            "#,
                client_key = client_key,
                amount = order.price_amount,
                order_id = order.order_number,
                prefix = payment::ORDER_NAME_PREFIX,
                filename = order.filename,
                success_url = success_url,
                fail_url = fail_url
            )))
        }
    }
}

pub fn checkout(
    current_user: &CurrentUser,
    flash: Option<&FlashMessage>,
    site_name: &str,
    order: &Order,
    client_key: &str,
) -> Markup {
    let success_url = "/actions/payment/verify".to_string();
    let fail_url = paths::helpers::quote_path(&order.order_id);

    let content = html! {
        div class="max-w-lg mx-auto" {
            h1 class="text-xl mb-3" { "Checkout" }

            div class="space-y-3" {
                div class="space-y-1 text-sm" {
                    div class="flex justify-between" {
                        span class="text-gray-600" { "File" }
                        span { (order.filename) }
                    }
                    div class="flex justify-between" {
                        span class="text-gray-600" { "Characters" }
                        span { (order.text_length.to_string()) }
                    }
                }

                div class="border-t pt-3 mb-3" {
                    div class="flex justify-between items-center" {
                        span { "Total" }
                        span class="text-xl text-indigo-600" { "₩" (format_price(order.price_amount)) }
                    }
                }

                div id="payment-method" class="mb-3" {}
                div id="agreement" class="mb-3" {}

                button
                    id="payment-button"
                    class="w-full bg-indigo-600 text-white px-3 py-2 hover:bg-indigo-700 disabled:bg-gray-400 disabled:cursor-not-allowed"
                    { "Pay Now" }
            }
        }

        (toss_payment_script(client_key, order, &success_url, &fail_url))
    };

    base::base_layout(current_user, flash, site_name, "Checkout", "Complete your payment", content)
}
