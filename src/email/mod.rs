mod config;
mod templates;

pub use config::{EmailConfig, EmailError, send_contact_inquiry, send_magic_link};
