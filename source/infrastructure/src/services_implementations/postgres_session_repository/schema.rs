use nimbus_auth_application::services::session_repository::errors::SessionRepositoryError;
use nimbus_auth_domain::{
    entities::{
        Entity,
        session::{
            InitializedSession, InitializedSessionRef, Session,
            specifications::RestoreSessionSpecification,
        },
    },
    value_objects::identifier::Identifier,
};
use nimbus_auth_shared::errors::ErrorBoxed;
use sqlx::prelude::FromRow;
use time::OffsetDateTime;
use ulid::Ulid;

#[derive(FromRow)]
pub struct GetSessionDb {
    id: String,
    user_id: String,
    expires_at: OffsetDateTime,
    revoked_at: Option<OffsetDateTime>,
}

#[derive(FromRow)]
pub struct SaveSessionDb {
    id: String,
    user_id: Option<String>,
    expires_at: Option<OffsetDateTime>,
    revoked_at: Option<OffsetDateTime>,
}

impl GetSessionDb {
    pub fn into_domain(
        self,
        current_time: OffsetDateTime,
    ) -> Result<InitializedSession, SessionRepositoryError> {
        Ok(Session::restore(RestoreSessionSpecification {
            id: Identifier::from(Ulid::from_string(&self.id).map_err(ErrorBoxed::from)?),
            user_id: Identifier::from(Ulid::from_string(&self.user_id).map_err(ErrorBoxed::from)?),
            expires_at: self.expires_at,
            revoked_at: self.revoked_at,
            current_time,
        }))
    }
}

impl From<InitializedSessionRef<'_>> for SaveSessionDb {
    fn from(value: InitializedSessionRef) -> Self {
        match value {
            InitializedSessionRef::Active(session) => SaveSessionDb {
                id: session.id().to_string(),
                user_id: Some(session.user_id().to_string()),
                expires_at: Some(session.expires_at()),
                revoked_at: None,
            },
            InitializedSessionRef::Expired(session) => SaveSessionDb {
                id: session.id().to_string(),
                user_id: None,
                expires_at: None,
                revoked_at: None,
            },
            InitializedSessionRef::Revoked(session) => SaveSessionDb {
                id: session.id().to_string(),
                user_id: None,
                expires_at: None,
                revoked_at: Some(session.revoked_at()),
            },
        }
    }
}
