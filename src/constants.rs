pub mod auth {
    pub const MAGIC_LINK_EXPIRY_MINUTES: i64 = 15;
    pub const SESSION_EXPIRY_DAYS: i64 = 1;
}

pub mod validation {
    use regex::Regex;
    use std::sync::LazyLock;

    pub static EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
            .expect("Email regex pattern is invalid")
    });
}

pub mod cdn {
    pub const TAILWIND_CSS_URL: &str = "https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4";
    pub const HTMX_URL: &str = "https://cdn.jsdelivr.net/npm/htmx.org@2.0.7/dist/htmx.min.js";
    pub const HTMX_INTEGRITY: &str = "sha384-ZBXiYtYQ6hJ2Y0ZNoYuI+Nq5MqWBr+chMrS/RkXpNzQCApHEhOt2aY8EJgqwHLkJ";
    pub const HYPERSCRIPT_URL: &str = "https://unpkg.com/hyperscript.org@0.9.14";
    pub const TOSS_PAYMENTS_SDK_URL: &str = "https://js.tosspayments.com/v1/payment";
}

pub mod error_pages {
    pub const FALLBACK_SITE_NAME: &str = "App";
}

pub mod messages {
    pub const MAGIC_LINK_SENT: &str = "Check your email! We sent you a link to sign in.";
    pub const SIGNED_IN: &str = "Successfully signed in!";
    pub const SIGNED_OUT: &str = "You have been signed out.";
    pub const TODO_CREATED: &str = "Todo created successfully";
    pub const EMAIL_SEND_FAILED: &str = "Failed to send email. Please try again.";
    pub const MAGIC_LINK_INVALID: &str = "Invalid or expired magic link. Please request a new one.";
    pub const CONTACT_SENT: &str = "Thank you for your message! We'll get back to you soon.";
    pub const PAYMENT_SUCCESS: &str = "Payment successful! Your order is complete.";
    pub const PAYMENT_FAILED: &str = "Payment failed. Please try again.";
    pub const SIGN_IN_REQUIRED: &str = "Please sign in to continue";
    pub const ORDER_ALREADY_PROCESSED: &str = "Order already processed";
    pub const ADMIN_ROLE_GRANTED: &str = "Admin role granted";
    pub const ADMIN_ROLE_REVOKED: &str = "Admin role revoked";
}

pub mod errors {
    pub const ORDER_NOT_FOUND: &str = "Order not found";
    pub const TODO_NOT_FOUND: &str = "Todo not found";
    pub const PAYMENT_NOT_COMPLETED: &str = "Payment not completed";
    pub const NOT_YOUR_ORDER: &str = "Not your order";
    pub const NO_FILE_PROVIDED: &str = "No file provided";
    pub const FORBIDDEN: &str = "You don't have permission to access this resource";
    pub const CANNOT_REVOKE_OWN_ADMIN: &str = "Cannot revoke your own admin role";
}

pub mod pricing {
    pub const PRICE_PER_CHARACTER: i32 = 1;
    pub const MINIMUM_ORDER_AMOUNT: i32 = 100;
}

pub mod payment {
    pub const TOSS_API_CONFIRM_URL: &str = "https://api.tosspayments.com/v1/payments/confirm";
    pub const ORDER_NAME_PREFIX: &str = "Text Analysis";
}

pub mod file_upload {
    pub const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB
}

pub mod logging {
    pub const UNKNOWN_CLIENT_IP: &str = "unknown";
}

pub mod admin {
    pub const ITEMS_PER_PAGE: i64 = 20;
}
