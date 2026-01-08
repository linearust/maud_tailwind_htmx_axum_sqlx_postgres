use crate::constants::auth::MAGIC_LINK_EXPIRY_MINUTES;

pub fn magic_link_sign_in(magic_link: &str) -> String {
    format!(
        r#"
        <html>
            <body style="font-family: sans-serif; max-width: 600px; margin: 0 auto; padding: 20px;">
                <h2>Sign in to your account</h2>
                <p>Click the link below to sign in. This link will expire in {} minutes.</p>
                <p style="margin: 30px 0;">
                    <a href="{}" style="background-color: #4F46E5; color: white; padding: 12px 24px; text-decoration: none; border-radius: 6px; display: inline-block;">
                        Sign In
                    </a>
                </p>
                <p style="color: #666; font-size: 14px;">
                    Or copy and paste this link into your browser:<br>
                    <a href="{}">{}</a>
                </p>
                <p style="color: #999; font-size: 12px; margin-top: 40px;">
                    If you didn't request this email, you can safely ignore it.
                </p>
            </body>
        </html>
        "#,
        MAGIC_LINK_EXPIRY_MINUTES, magic_link, magic_link, magic_link
    )
}

pub fn contact_inquiry(email: &str, message: &str) -> String {
    format!(
        r#"
        <html>
            <body style="font-family: sans-serif; max-width: 600px; margin: 0 auto; padding: 20px;">
                <h2>New Contact Inquiry</h2>
                <p><strong>From:</strong> {}</p>
                <div style="margin: 20px 0; padding: 15px; background-color: #f5f5f5; border-radius: 6px;">
                    <p style="margin: 0; white-space: pre-wrap;">{}</p>
                </div>
            </body>
        </html>
        "#,
        email, message
    )
}
