use axum::{extract::Request, http::{HeaderValue, header}, middleware::Next};

const CSP_POLICY: &str = "default-src 'self'; script-src 'self' 'unsafe-inline' https://unpkg.com https://cdn.jsdelivr.net https://js.tosspayments.com https://*.tosspayments.com; style-src 'self' 'unsafe-inline' https://cdn.jsdelivr.net; connect-src 'self' https://cdn.jsdelivr.net https://api.tosspayments.com https://*.tosspayments.com; img-src 'self' data: https://*.tosspayments.com; font-src 'self' https://cdn.jsdelivr.net; frame-src https://api.tosspayments.com https://*.tosspayments.com; form-action 'self' https://*.tosspayments.com";

pub async fn security_headers(req: Request, next: Next) -> axum::response::Response {
    let mut res = next.run(req).await;
    let headers = res.headers_mut();

    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );

    headers.insert(header::X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));

    headers.insert(
        "X-XSS-Protection",
        HeaderValue::from_static("1; mode=block"),
    );

    headers.insert(
        header::STRICT_TRANSPORT_SECURITY,
        HeaderValue::from_static("max-age=31536000; includeSubDomains"),
    );

    headers.insert(
        header::REFERRER_POLICY,
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );

    headers.insert(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static(CSP_POLICY),
    );

    headers.insert(
        "Permissions-Policy",
        HeaderValue::from_static("geolocation=(), microphone=(), camera=()"),
    );

    res
}
