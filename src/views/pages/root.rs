use maud::{html, Markup};

use crate::{
    auth::CurrentUser,
    paths,
    session::FlashMessage,
    views::{components::form, layout::base::base_layout},
};

pub fn root(
    current_user: &CurrentUser,
    flash: Option<&FlashMessage>,
    site_name: &str,
    email: Option<&str>,
    message: Option<&str>,
    email_error: Option<&str>,
    message_error: Option<&str>,
) -> Markup {
    let is_readonly = current_user.is_authenticated();

    let content = html! {
        div class="max-w-lg mx-auto" {
            h1 class="text-xl mb-3" { "Contact" }

            form method="post" action=(paths::forms::CONTACT) class="space-y-3" {
                (form::input_with_label("email", "email", Some("Email"), "your@email.com", email, email_error, is_readonly))

                div {
                    label for="message" class="block text-sm mb-1" { "Message" }
                    textarea
                        id="message"
                        name="message"
                        required
                        rows="5"
                        class="w-full px-3 py-2 border focus:outline-none focus:border-indigo-600"
                        placeholder="Your message..." { (message.unwrap_or("")) }
                    @if let Some(error) = message_error {
                        p class="text-red-600 text-sm mt-1" { (error) }
                    }
                }

                (form::submit_button("Send Message"))
            }
        }
    };

    base_layout(current_user, flash, site_name, "Home", "Home page", content)
}
