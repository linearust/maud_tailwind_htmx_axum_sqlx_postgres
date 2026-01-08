use crate::{auth::CurrentUser, session::FlashMessage, views::layout::base::base_layout};
use maud::{Markup, html};

pub fn not_found(current_user: &CurrentUser, flash: Option<&FlashMessage>, site_name: &str) -> Markup {
    let content = html! {
        h1 class="text-6xl" { "404" }
    };

    base_layout(current_user, flash, site_name, "Page Not Found", "Page not found", content)
}
