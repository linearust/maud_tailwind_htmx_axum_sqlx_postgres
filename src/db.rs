use std::sync::LazyLock;
use surrealdb::{engine::any::Any, Surreal};

pub static DB: LazyLock<Surreal<Any>> = LazyLock::new(Surreal::init);
