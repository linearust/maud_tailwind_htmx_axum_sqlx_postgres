use serde::Deserialize;
use validator::Validate;

use crate::constants::validation::EMAIL_REGEX;

pub const FIELD_EMAIL: &str = "email";
pub const FIELD_MESSAGE: &str = "message";

#[derive(Deserialize, Validate)]
pub struct ContactForm {
    #[validate(regex(path = "*EMAIL_REGEX", message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 1, message = "Message cannot be empty"))]
    pub message: String,
}
