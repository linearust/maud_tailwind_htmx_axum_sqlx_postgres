use crate::{auth::CurrentUser, session::FlashMessage, paths, views::layout::base::base_layout};
use maud::{Markup, html};

pub fn text_analyzer(
    current_user: &CurrentUser,
    flash: Option<&FlashMessage>,
    site_name: &str,
) -> Markup {
    let content = html! {
        div class="max-w-lg mx-auto" {
            h1 class="text-xl mb-3" { "Text Analyzer" }

            form method="post" action=(paths::forms::TEXT_ANALYZER) enctype="multipart/form-data" class="space-y-3" {
                div {
                    label for="file" class="block text-sm mb-1" {
                        "Text File"
                    }
                    input
                        type="file"
                        id="file"
                        name="file"
                        accept=".txt"
                        required
                        class="w-full px-3 py-2 border";
                }

                button
                    type="submit"
                    class="w-full bg-indigo-600 text-white px-3 py-2 hover:bg-indigo-700"
                    { "Get Quote" }
            }
        }
    };

    base_layout(current_user, flash, site_name, "Text Analyzer", "Upload files for text analysis", content)
}
