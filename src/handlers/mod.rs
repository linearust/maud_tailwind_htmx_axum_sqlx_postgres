//! HTTP request handlers organized by interaction type.
//!
//! Handlers are grouped by the type of interaction (pages, forms, actions)
//! rather than by resource, mirroring the route organization pattern.

pub mod actions;
pub mod errors;
pub mod fallback;
pub mod forms;
pub mod htmx;
pub mod pages;
