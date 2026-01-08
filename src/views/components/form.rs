use maud::{Markup, html};

pub fn input(
    input_type: &str,
    name: &str,
    placeholder: &str,
    value: Option<&str>,
    error: Option<&str>,
) -> Markup {
    input_with_label(input_type, name, None, placeholder, value, error, false)
}

pub fn input_with_label(
    input_type: &str,
    name: &str,
    label: Option<&str>,
    placeholder: &str,
    value: Option<&str>,
    error: Option<&str>,
    readonly: bool,
) -> Markup {
    let input_class = match (error.is_some(), readonly) {
        (true, _) => "w-full px-3 py-2 border border-red-500 focus:outline-none focus:border-red-600",
        (false, true) => "w-full px-3 py-2 border bg-gray-50 cursor-not-allowed",
        (false, false) => "w-full px-3 py-2 border focus:outline-none focus:border-indigo-600",
    };

    html! {
        div {
            @if let Some(label_text) = label {
                label for=(name) class="block text-sm mb-1" { (label_text) }
            }
            @if readonly {
                input type=(input_type) name=(name) id=(name) readonly
                    class=(input_class)
                    placeholder=(placeholder)
                    value=[value];
            } @else {
                input type=(input_type) name=(name) id=(name) required
                    class=(input_class)
                    placeholder=(placeholder)
                    value=[value];
            }

            @if let Some(error_msg) = error {
                p class="mt-1 text-sm text-red-600" { (error_msg) }
            }
        }
    }
}

pub fn submit_button(text: &str) -> Markup {
    html! {
        button type="submit" class="w-full bg-indigo-600 text-white px-3 py-2 hover:bg-indigo-700" {
            (text)
        }
    }
}
