use crate::{
    auth::CurrentUser,
    flash::FlashMessage,
    models::sign_in::FIELD_EMAIL,
    paths,
    views::{components::form, layout::base::base_layout},
};
use maud::{html, Markup};

pub fn sign_in(
    current_user: &CurrentUser,
    flash: Option<&FlashMessage>,
    site_name: &str,
    email_value: Option<&str>,
    email_error: Option<&str>,
) -> Markup {
    let content = html! {
        div class="max-w-sm mx-auto" {
            h1 class="text-xl mb-3" { "Sign In" }

            form method="POST" action=(paths::forms::SIGN_IN) class="space-y-3" {
                (form::input("email", FIELD_EMAIL, "Email", email_value, email_error))
                (form::submit_button("Send Magic Link"))
            }
        }
    };

    base_layout(current_user, flash, site_name, "Sign In", "Sign in", content)
}
