/// Generates absolute and relative path constants from a base prefix.
macro_rules! define_nested_routes {
    ($base:expr, { $($name:ident => $path:expr),* $(,)? }) => {
        pub const BASE: &str = $base;

        #[allow(dead_code)]
        pub mod relative {
            $(pub const $name: &str = $path;)*
        }

        $(
            #[allow(dead_code)]
            pub const $name: &str = concat!($base, $path);
        )*
    };
}

pub mod pages {
    pub const ROOT: &str = "/";
    pub const SIGN_IN: &str = "/sign_in";
    pub const DASHBOARD: &str = "/dashboard";
    pub const TODOS: &str = "/todos";
    pub const TEXT_ANALYZER: &str = "/text_analyzer";
    pub const QUOTE: &str = "/quote/{order_id}";
    pub const CHECKOUT: &str = "/checkout/{order_id}";
    pub const PAYMENT_CONFIRMATION: &str = "/payment_confirmation/{order_id}";

    pub mod admin {
        pub const HOME: &str = "/admin";
        pub const USERS: &str = "/admin/users";
        pub const USER_DETAIL: &str = "/admin/users/{user_id}";
        pub const ORDERS: &str = "/admin/orders";
        pub const ORDER_DETAIL: &str = "/admin/orders/{order_id}";
    }
}

pub mod forms {
    define_nested_routes!("/forms", {
        SIGN_IN => "/sign_in",
        TODOS => "/todos",
        CONTACT => "/contact",
        TEXT_ANALYZER => "/text_analyzer",
    });

    pub mod admin {
        pub const GRANT_ROLE: &str = "/forms/admin/users/{user_id}/grant-role";
    }
}

pub mod actions {
    define_nested_routes!("/actions", {
        SIGN_OUT => "/sign_out",
        VERIFY_MAGIC_LINK => "/auth/verify",
        TODOS_TODO_ID => "/todos/{todo_id}",
        TODOS_TODO_ID_TOGGLE => "/todos/{todo_id}/toggle",
        PAYMENT_INITIATE => "/payment/initiate",
        PAYMENT_VERIFY => "/payment/verify",
    });

    pub mod admin {
        pub const REVOKE_ROLE: &str = "/actions/admin/users/{user_id}/revoke-role";
    }
}

pub mod static_files {
    define_nested_routes!("/static", {
        FAVICON => "/img/favicon.svg",
    });
}

pub fn with_param(path: &str, param_name: &str, value: &impl ToString) -> String {
    path.replace(&format!("{{{}}}", param_name), &value.to_string())
}

pub fn with_query_param(base: &str, key: &str, value: &str) -> String {
    format!("{}?{}={}", base, key, value)
}

pub fn with_page(base: &str, page: i64) -> String {
    with_query_param(base, "page", &page.to_string())
}

pub mod helpers {
    use super::*;
    use crate::models::{OrderId, UserId};

    pub fn user_detail_path(user_id: &UserId) -> String {
        with_param(pages::admin::USER_DETAIL, "user_id", user_id)
    }

    pub fn order_detail_path(order_id: impl ToString) -> String {
        with_param(pages::admin::ORDER_DETAIL, "order_id", &order_id)
    }

    pub fn quote_path(order_id: &OrderId) -> String {
        with_param(pages::QUOTE, "order_id", order_id)
    }

    pub fn checkout_path(order_id: &OrderId) -> String {
        with_param(pages::CHECKOUT, "order_id", order_id)
    }

    pub fn payment_confirmation_path(order_id: &OrderId) -> String {
        with_param(pages::PAYMENT_CONFIRMATION, "order_id", order_id)
    }
}
