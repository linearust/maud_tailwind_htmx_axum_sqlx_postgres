pub mod admin;
mod checkout;
mod dashboard;
mod quote;
mod payment_confirmation;
mod root;
mod sign_in;
mod text_analyzer;
mod todos;

pub use checkout::get_checkout;
pub use dashboard::get_dashboard;
pub use quote::get_quote;
pub use payment_confirmation::get_payment_confirmation;
pub use root::get_root;
pub use sign_in::get_sign_in;
pub use text_analyzer::get_text_analyzer;
pub use todos::get_todos;
