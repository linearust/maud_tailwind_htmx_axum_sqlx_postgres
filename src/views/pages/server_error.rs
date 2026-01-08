use crate::{auth::CurrentUser, session::FlashMessage, views::layout::base::base_layout};
use maud::{Markup, html};

pub fn server_error(current_user: &CurrentUser, flash: Option<&FlashMessage>, site_name: &str, message: &str) -> Markup {
    let content = html! {
        h1 class="text-6xl mb-3" { "500" }
        p class="text-red-600" { (message) }
    };

    base_layout(current_user, flash, site_name, "Server Error", "Server error", content)
}
