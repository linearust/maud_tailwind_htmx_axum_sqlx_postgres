use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;
use surrealdb::sql::{Bytes, Datetime};
use time::OffsetDateTime;
use tower_sessions::{
    session::{Id, Record},
    session_store, ExpiredDeletion, SessionStore,
};

use crate::db::DB;

fn time_to_chrono(t: OffsetDateTime) -> DateTime<Utc> {
    DateTime::<Utc>::from_timestamp(t.unix_timestamp(), t.nanosecond())
        .expect("Valid timestamp from tower-sessions")
}

fn chrono_to_time(c: DateTime<Utc>) -> OffsetDateTime {
    OffsetDateTime::from_unix_timestamp(c.timestamp())
        .expect("Valid timestamp from SurrealDB")
}

#[derive(Clone, Debug)]
pub struct SurrealSessionStore;

impl SurrealSessionStore {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Serialize, Deserialize)]
struct SessionData {
    data: Bytes,
    expires_at: Datetime,
}

#[async_trait]
impl SessionStore for SurrealSessionStore {
    async fn create(&self, record: &mut Record) -> session_store::Result<()> {
        let data = rmp_serde::to_vec(&record).map_err(|e| {
            session_store::Error::Encode(e.to_string())
        })?;
        let session_data = SessionData {
            data: Bytes::from(data),
            expires_at: Datetime::from(time_to_chrono(record.expiry_date)),
        };

        let record_id = RecordId::from(("session", record.id.to_string()));
        let _: Option<SessionData> = DB
            .create(record_id)
            .content(session_data)
            .await
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;

        Ok(())
    }

    async fn save(&self, record: &Record) -> session_store::Result<()> {
        let data = rmp_serde::to_vec(record).map_err(|e| {
            session_store::Error::Encode(e.to_string())
        })?;
        let session_data = SessionData {
            data: Bytes::from(data),
            expires_at: Datetime::from(time_to_chrono(record.expiry_date)),
        };

        let record_id = RecordId::from(("session", record.id.to_string()));
        let _: Option<SessionData> = DB
            .update(record_id)
            .content(session_data)
            .await
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;

        Ok(())
    }

    async fn load(&self, session_id: &Id) -> session_store::Result<Option<Record>> {
        let record_id = RecordId::from(("session", session_id.to_string()));
        let session_data: Option<SessionData> = DB
            .select(record_id)
            .await
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;

        match session_data {
            Some(s) => {
                let expires_chrono: DateTime<Utc> = s.expires_at.into();
                let expires = chrono_to_time(expires_chrono);
                if expires > OffsetDateTime::now_utc() {
                    let session: Record = rmp_serde::from_slice(s.data.as_slice()).map_err(|e| {
                        session_store::Error::Decode(e.to_string())
                    })?;
                    Ok(Some(session))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }

    async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
        let record_id = RecordId::from(("session", session_id.to_string()));
        let _: Option<SessionData> = DB
            .delete(record_id)
            .await
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;

        Ok(())
    }
}

#[async_trait]
impl ExpiredDeletion for SurrealSessionStore {
    async fn delete_expired(&self) -> session_store::Result<()> {
        DB.query("DELETE session WHERE expires_at <= time::now()")
            .await
            .map_err(|e| session_store::Error::Backend(e.to_string()))?;

        Ok(())
    }
}
