pub mod admin;
mod contact;
mod sign_in;
mod text_analyzer;
mod todos;

pub use contact::post_forms_contact;
pub use sign_in::post_forms_sign_in;
pub use text_analyzer::post_forms_text_analyzer;
pub use todos::post_forms_todos;

use std::collections::HashMap;

pub(super) fn parse_validation_errors(
    validation_errors: &validator::ValidationErrors,
) -> HashMap<String, String> {
    validation_errors
        .field_errors()
        .iter()
        .filter_map(|(field, errs)| {
            errs.first()
                .and_then(|e| e.message.as_ref())
                .map(|msg| (field.to_string(), msg.to_string()))
        })
        .collect()
}
