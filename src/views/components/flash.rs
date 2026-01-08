use maud::{html, Markup};

use crate::session::{FlashKind, FlashMessage};

pub fn flash(message: Option<&FlashMessage>) -> Markup {
    match message {
        Some(flash) => {
            let text_color = match flash.kind {
                FlashKind::Success => "text-green-700",
                FlashKind::Error => "text-red-700",
                FlashKind::Info => "text-indigo-700",
            };

            html! {
                div
                    id="flash-message"
                    class={"mb-4 p-3 " (text_color)}
                    _="on load wait 5s then transition opacity to 0 over 500ms then remove me"
                {
                    (flash.message)
                }
            }
        }
        None => html! {},
    }
}
