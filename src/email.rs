use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};

use crate::{email_templates, paths};

#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("Email build error: {0}")]
    Build(#[from] lettre::error::Error),
    #[error("Email transport error: {0}")]
    Transport(#[from] lettre::transport::smtp::Error),
    #[error("Invalid email address: {0}")]
    InvalidAddress(#[from] lettre::address::AddressError),
    #[error("Configuration error: {0}")]
    Config(String),
}

#[derive(Clone)]
pub struct EmailConfig {
    mode: EmailMode,
    from_address: String,
    from_name: String,
    base_url: String,
}

#[derive(Clone)]
pub enum EmailMode {
    Console,
    Smtp {
        host: String,
        port: u16,
        username: String,
        password: String,
    },
}

impl EmailConfig {
    pub fn from_env() -> Result<Self, EmailError> {
        let mode_str = dotenvy::var("EMAIL_MODE")
            .map_err(|_| EmailError::Config("EMAIL_MODE must be set".to_string()))?;

        let mode = match mode_str.as_str() {
            "console" => EmailMode::Console,
            "smtp" => {
                let host = dotenvy::var("SMTP_HOST")
                    .map_err(|_| EmailError::Config("SMTP_HOST must be set when EMAIL_MODE=smtp".to_string()))?;
                let port = dotenvy::var("SMTP_PORT")
                    .map_err(|_| EmailError::Config("SMTP_PORT must be set when EMAIL_MODE=smtp".to_string()))?
                    .parse()
                    .map_err(|_| EmailError::Config("SMTP_PORT must be a valid number".to_string()))?;
                let username = dotenvy::var("SMTP_USERNAME")
                    .map_err(|_| EmailError::Config("SMTP_USERNAME must be set when EMAIL_MODE=smtp".to_string()))?;
                let password = dotenvy::var("SMTP_PASSWORD")
                    .map_err(|_| EmailError::Config("SMTP_PASSWORD must be set when EMAIL_MODE=smtp".to_string()))?;

                EmailMode::Smtp {
                    host,
                    port,
                    username,
                    password,
                }
            }
            _ => return Err(EmailError::Config(format!("EMAIL_MODE must be either 'console' or 'smtp', got '{}'", mode_str))),
        };

        let from_address = dotenvy::var("EMAIL_FROM_ADDRESS")
            .map_err(|_| EmailError::Config("EMAIL_FROM_ADDRESS must be set".to_string()))?;
        let from_name = dotenvy::var("EMAIL_FROM_NAME")
            .map_err(|_| EmailError::Config("EMAIL_FROM_NAME must be set".to_string()))?;
        let base_url = dotenvy::var("BASE_URL")
            .map_err(|_| EmailError::Config("BASE_URL must be set".to_string()))?;

        Ok(Self {
            mode,
            from_address,
            from_name,
            base_url,
        })
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    fn create_smtp_transport(&self) -> Result<SmtpTransport, EmailError> {
        match &self.mode {
            EmailMode::Smtp { host, port, username, password } => {
                let creds = Credentials::new(username.clone(), password.clone());
                Ok(SmtpTransport::starttls_relay(host)?
                    .port(*port)
                    .credentials(creds)
                    .build())
            }
            EmailMode::Console => unreachable!("Console mode doesn't need SMTP transport"),
        }
    }
}

pub async fn send_magic_link(
    config: &EmailConfig,
    to_email: &str,
    token: &str,
) -> Result<(), EmailError> {
    let magic_link = format!("{}{}?token={}", config.base_url, paths::actions::VERIFY_MAGIC_LINK, token);

    let from_mailbox: Mailbox = format!("{} <{}>", config.from_name, config.from_address).parse()?;
    let to_mailbox: Mailbox = to_email.parse()?;

    let email = Message::builder()
        .from(from_mailbox)
        .to(to_mailbox)
        .subject("Sign in to your account")
        .header(ContentType::TEXT_HTML)
        .body(email_templates::magic_link_sign_in(&magic_link))?;

    match &config.mode {
        EmailMode::Console => {
            tracing::info!("\n\n========== MAGIC LINK EMAIL ==========");
            tracing::info!("To: {}", to_email);
            tracing::info!("Magic Link: {}", magic_link);
            tracing::info!("======================================\n");
            Ok(())
        }
        EmailMode::Smtp { .. } => {
            let mailer = config.create_smtp_transport()?;
            mailer.send(&email)?;
            tracing::info!("Magic link email sent to {}", to_email);
            Ok(())
        }
    }
}

pub async fn send_contact_inquiry(
    config: &EmailConfig,
    from_email: &str,
    message: &str,
) -> Result<(), EmailError> {
    let from_mailbox: Mailbox = format!("{} <{}>", config.from_name, config.from_address).parse()?;
    let to_mailbox: Mailbox = config.from_address.parse()?;

    let email = Message::builder()
        .from(from_mailbox)
        .to(to_mailbox)
        .subject("New Contact Inquiry")
        .header(ContentType::TEXT_HTML)
        .body(email_templates::contact_inquiry(from_email, message))?;

    match &config.mode {
        EmailMode::Console => {
            tracing::info!("\n\n========== CONTACT INQUIRY EMAIL ==========");
            tracing::info!("From: {}", from_email);
            tracing::info!("Message: {}", message);
            tracing::info!("===========================================\n");
            Ok(())
        }
        EmailMode::Smtp { .. } => {
            let mailer = config.create_smtp_transport()?;
            mailer.send(&email)?;
            tracing::info!("Contact inquiry email sent from {}", from_email);
            Ok(())
        }
    }
}
