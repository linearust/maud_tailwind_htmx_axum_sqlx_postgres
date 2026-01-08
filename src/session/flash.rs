use serde::{Deserialize, Serialize};

const FLASH_MESSAGE_KEY: &str = "_flash";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlashMessage {
    pub message: String,
    pub kind: FlashKind,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FlashKind {
    Success,
    Error,
    Info,
}

impl FlashMessage {
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            kind: FlashKind::Success,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            kind: FlashKind::Error,
        }
    }

    pub fn info(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            kind: FlashKind::Info,
        }
    }

    pub async fn set(
        self,
        session: &tower_sessions::Session,
    ) -> Result<(), tower_sessions::session::Error> {
        session.insert(FLASH_MESSAGE_KEY, self).await
    }

    pub async fn set_and_redirect(
        self,
        session: &tower_sessions::Session,
        path: &str,
    ) -> Result<axum::response::Response, tower_sessions::session::Error> {
        use axum::response::{IntoResponse, Redirect};
        self.set(session).await?;
        Ok(Redirect::to(path).into_response())
    }

    pub async fn get(
        session: &tower_sessions::Session,
    ) -> Result<Option<Self>, tower_sessions::session::Error> {
        session.remove(FLASH_MESSAGE_KEY).await
    }
}
