use axum::extract::FromRef;

use crate::email::EmailConfig;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing required environment variable: {0}")]
    MissingVar(String),
    #[error("Email configuration error: {0}")]
    Email(#[from] crate::email::EmailError),
}

#[derive(Clone)]
pub struct PaymentConfig {
    toss_client_key: String,
    toss_secret_key: String,
}

impl PaymentConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let toss_client_key = dotenvy::var("TOSS_CLIENT_KEY")
            .map_err(|_| ConfigError::MissingVar("TOSS_CLIENT_KEY".to_string()))?;

        let toss_secret_key = dotenvy::var("TOSS_SECRET_KEY")
            .map_err(|_| ConfigError::MissingVar("TOSS_SECRET_KEY".to_string()))?;

        Ok(Self {
            toss_client_key,
            toss_secret_key,
        })
    }

    pub fn toss_client_key(&self) -> &str {
        &self.toss_client_key
    }

    pub fn toss_secret_key(&self) -> &str {
        &self.toss_secret_key
    }
}

#[derive(Clone)]
pub struct AppConfig {
    server_addr: String,
    database_url: String,
    site_name: String,
    email: EmailConfig,
    payment: PaymentConfig,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let database_url = dotenvy::var("DATABASE_URL")
            .map_err(|_| ConfigError::MissingVar("DATABASE_URL".to_string()))?;

        let server_addr = dotenvy::var("SERVER_ADDR")
            .map_err(|_| ConfigError::MissingVar("SERVER_ADDR".to_string()))?;

        let site_name = dotenvy::var("SITE_NAME")
            .map_err(|_| ConfigError::MissingVar("SITE_NAME".to_string()))?;

        let email = EmailConfig::from_env()?;
        let payment = PaymentConfig::from_env()?;

        Ok(Self {
            server_addr,
            database_url,
            site_name,
            email,
            payment,
        })
    }

    pub fn server_addr(&self) -> &str {
        &self.server_addr
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }

    pub fn site_name(&self) -> &str {
        &self.site_name
    }

    pub fn email(&self) -> &EmailConfig {
        &self.email
    }

    pub fn payment(&self) -> &PaymentConfig {
        &self.payment
    }
}

#[derive(Clone, FromRef)]
pub struct AppState {
    config: AppConfig,
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }
}
