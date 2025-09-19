use std::sync::Arc;

use nimbus_auth_domain::{
    entities::{
        Entity,
        session::{Session, SomeSession, SomeSessionRef},
    },
    value_objects::identifier::Identifier,
};
use nimbus_auth_shared::types::{AccessTokenExpirationSeconds, SessionExpirationSeconds};
use ulid::Ulid;

use crate::{
    services::{
        keypair_repository::KeyPairRepository, session_repository::SessionRepository,
        time_service::TimeService, user_repository::UserRepository,
    },
    use_cases::{RefreshRequest, RefreshResponse, UserDto, refresh::errors::RefreshError},
};

pub mod errors;
pub mod schema;

pub async fn handle_refresh(
    RefreshRequest { session_id }: RefreshRequest,
    user_repository: Arc<dyn UserRepository>,
    session_repository: Arc<dyn SessionRepository>,
    keypair_repository: Arc<dyn KeyPairRepository>,
    time_service: Arc<dyn TimeService>,
    session_exp_seconds: SessionExpirationSeconds,
    access_token_exp_seconds: AccessTokenExpirationSeconds,
) -> Result<RefreshResponse, RefreshError> {
    let session = session_repository
        .get_by_id(Identifier::from(Ulid::from_string(&session_id)?))
        .await?
        .ok_or(RefreshError::SessionIsNotFound)?;

    let active_session = match session {
        SomeSession::Active { session, .. } => Ok(session),
        SomeSession::Expired { .. } => Err(RefreshError::SessionIsExpired),
        SomeSession::Revoked { .. } => Err(RefreshError::SessionIsRevoked),
    }?;

    let user = user_repository
        .get_by_session(&active_session)
        .await?
        .ok_or(RefreshError::UserIsNotFound)?;

    let active_keypair = keypair_repository
        .get_active()
        .await?
        .ok_or(RefreshError::ActiveKeyPairNotFound)?;

    let (revoked_session, new_active_session) =
        active_session.refresh(time_service.get_current_time().await?, session_exp_seconds);

    let transactional_session_repository = session_repository.start_transaction().await?;

    let (transactional_session_repository, _) = transactional_session_repository
        .save(SomeSessionRef::Revoked(&revoked_session))
        .await?;

    let (transactional_session_repository, _) = transactional_session_repository
        .save(SomeSessionRef::Active(&new_active_session))
        .await?;

    let access_token = &new_active_session.generate_access_token(
        time_service.get_current_time().await?,
        access_token_exp_seconds,
    );
    let signed_access_token = access_token.sign(&active_keypair)?;

    transactional_session_repository.commit().await?;

    Ok(RefreshResponse {
        user: UserDto::from(&user),
        session_id: new_active_session.id().to_string(),
        signed_access_token,
    })
}
