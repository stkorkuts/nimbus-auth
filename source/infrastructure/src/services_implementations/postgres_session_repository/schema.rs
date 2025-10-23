use nimbus_auth_application::services::session_repository::errors::SessionRepositoryError;
use nimbus_auth_domain::{
    entities::{
        Entity,
        session::{SomeSession, specifications::RestoreSessionSpecification},
        user::value_objects::user_name::UserName,
    },
    value_objects::{identifier::Identifier, user_claims::UserClaims},
};
use nimbus_auth_shared::{errors::ErrorBoxed, types::UserRole};
use sqlx::prelude::FromRow;
use time::OffsetDateTime;
use ulid::Ulid;

use crate::{
    postgres_db::types::user_role::UserRoleDb,
    services_implementations::postgres_session_repository::schema::errors::SessionDbIntoDomainError,
};

pub mod errors;

#[derive(FromRow)]
pub struct GetSessionDb {
    id: String,
    user_id: String,
    user_name: String,
    user_role: UserRoleDb,
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
    ) -> Result<SomeSession<'static>, SessionDbIntoDomainError> {
        let user_id = Identifier::from(Ulid::from_string(&self.user_id)?);
        let user_name = UserName::from(&self.user_name)?;
        let user_role = UserRole::from(&self.user_role);
        Ok(SomeSession::restore(RestoreSessionSpecification {
            id: Identifier::from(Ulid::from_string(&self.id)?),
            user_claims: UserClaims::new(user_id, user_name, user_role),
            expires_at: self.expires_at,
            revoked_at: self.revoked_at,
            current_time,
        }))
    }
}

impl<'a> From<SomeSession<'a>> for SaveSessionDb {
    fn from(value: SomeSession<'a>) -> Self {
        match value {
            SomeSession::Active(session) => SaveSessionDb {
                id: session.id().to_string(),
                user_id: Some(session.user_claims().id().to_string()),
                expires_at: Some(session.expires_at()),
                revoked_at: None,
            },
            SomeSession::Expired(session) => SaveSessionDb {
                id: session.id().to_string(),
                user_id: None,
                expires_at: None,
                revoked_at: None,
            },
            SomeSession::Revoked(session) => SaveSessionDb {
                id: session.id().to_string(),
                user_id: None,
                expires_at: None,
                revoked_at: Some(session.revoked_at()),
            },
        }
    }
}
