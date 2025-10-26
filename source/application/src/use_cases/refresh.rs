use std::{borrow::Cow, sync::Arc};

use nimbus_auth_domain::{
    entities::{Entity, session::SomeSession},
    value_objects::identifier::Identifier,
};
use nimbus_auth_shared::types::{AccessTokenExpirationSeconds, SessionExpirationSeconds};
use ulid::Ulid;
use zeroize::Zeroizing;

use crate::{
    services::{
        keypair_repository::KeyPairRepository, session_repository::SessionRepository,
        time_service::TimeService, user_repository::UserRepository,
    },
    use_cases::{
        RefreshRequest, RefreshResponse, UserClaimsDto,
        dtos::{access_token::AccessTokenDto, session::SessionDto},
        refresh::errors::RefreshError,
    },
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
        .get_by_id(&Identifier::from(Ulid::from_string(&session_id)?))
        .await?
        .ok_or(RefreshError::SessionIsNotFound)?;

    let active_session = match session {
        SomeSession::Active(session) => Ok(session),
        SomeSession::Expired(_) => Err(RefreshError::SessionIsExpired),
        SomeSession::Revoked(_) => Err(RefreshError::SessionIsRevoked),
    }?
    .into_owned();

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
        .save(SomeSession::Revoked(Cow::Borrowed(&revoked_session)))
        .await?;

    let (transactional_session_repository, _) = transactional_session_repository
        .save(SomeSession::Active(Cow::Borrowed(&new_active_session)))
        .await?;

    let access_token = &new_active_session.generate_access_token(
        time_service.get_current_time().await?,
        access_token_exp_seconds,
    );
    let signed_access_token = access_token.sign(&active_keypair)?;

    transactional_session_repository.commit().await?;

    let user_dto = UserClaimsDto::from(user.claims());

    let session_dto = SessionDto {
        session_id: Zeroizing::new(new_active_session.id().to_string()),
        session_expires_at_unix_timestamp: new_active_session.expires_at().unix_timestamp(),
    };

    let access_token_dto = AccessTokenDto {
        signed_access_token,
        signed_access_token_expires_at_unix_timestamp: access_token.expires_at().unix_timestamp(),
    };

    Ok(RefreshResponse {
        user: user_dto,
        session: session_dto,
        access_token: access_token_dto,
    })
}
