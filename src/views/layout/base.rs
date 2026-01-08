use super::navigation;
use crate::{auth::CurrentUser, constants::cdn, session::FlashMessage, paths, views::components};
use maud::{html, Markup, DOCTYPE};

pub fn base_layout(current_user: &CurrentUser, flash: Option<&FlashMessage>, site_name: &str, title: &str, meta_description: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (title) " - " (site_name) }
                meta name="description" content=(meta_description);

                link rel="icon" type="image/svg+xml" href=(paths::static_files::FAVICON);

                script src=(cdn::TAILWIND_CSS_URL) {}

                script src=(cdn::HTMX_URL)
                    integrity=(cdn::HTMX_INTEGRITY)
                    crossorigin="anonymous" {}

                script src=(cdn::HYPERSCRIPT_URL) {}
            }
            body class="min-h-screen flex flex-col" {
                (navigation::navbar(current_user))
                main class="flex-grow container mx-auto px-4 py-8" {
                    (components::flash::flash(flash))
                    (content)
                }
            }
        }
    }
}
