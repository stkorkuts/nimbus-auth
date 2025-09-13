use std::sync::Arc;

use nimbus_auth_domain::{
    entities::{
        Entity,
        session::{Active, InitializedSession, InitializedSessionRef, Session},
    },
    value_objects::identifier::{Identifier, IdentifierOfType},
};
use nimbus_auth_shared::{
    errors::ErrorBoxed,
    types::{AccessTokenExpirationSeconds, SessionExpirationSeconds},
};
use ulid::Ulid;

use crate::{
    services::{
        keypair_repository::KeyPairRepository, session_repository::SessionRepository,
        time_service::TimeService, user_repository::UserRepository,
    },
    use_cases::{
        RefreshRequest, RefreshResponse, UserDto,
        refresh::{self, errors::RefreshError},
        signup::errors::SignUpError,
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
    todo!();

    // let session_id = Identifier::from(Ulid::from_string(&session_id)?);

    // let session = session_repository
    //     .get_by_id(session_id.clone(), None)
    //     .await?
    //     .ok_or(RefreshError::SessionIsNotFound)?;

    // let active_session = (match session {
    //     InitializedSession::Expired(_) => Err(RefreshError::SessionIsExpired),
    //     InitializedSession::Revoked(_) => Err(RefreshError::SessionIsRevoked),
    //     InitializedSession::Active(active_session) => Ok(active_session),
    // })?;

    // let user = user_repository
    //     .get_by_session(&active_session, None)
    //     .await?
    //     .ok_or(RefreshError::UserIsNotFound)?;

    // let active_keypair = keypair_repository
    //     .get_active(None)
    //     .await?
    //     .ok_or(RefreshError::ActiveKeyPairNotFound)?;

    // let (revoked_session, active_session) = Session::refresh(
    //     active_session,
    //     time_service.get_current_time().await?,
    //     session_exp_seconds,
    // );
    // let active_session = Arc::new(active_session);

    // let mut session_repo_transaction = session_repository
    //     .start_transaction(
    //         TransactionIsolationLevel::Default,
    //         TransactonBlockTarget::Default,
    //     )
    //     .await?;

    // let active_session_for_transaction = active_session.clone();

    // let signed_access_token = session_repo_transaction
    //     .run(async move |inner_session_repo_transaction| {
    //         session_repository
    //             .save(
    //                 InitializedSessionRef::from(&revoked_session),
    //                 Some(inner_session_repo_transaction.clone()),
    //             )
    //             .await?;

    //         session_repository
    //             .save(
    //                 InitializedSessionRef::from(active_session_for_transaction.clone().as_ref()),
    //                 Some(inner_session_repo_transaction),
    //             )
    //             .await?;

    //         let access_token = active_session_for_transaction
    //             .clone()
    //             .generate_access_token(
    //                 time_service.get_current_time().await?,
    //                 access_token_exp_seconds,
    //             );
    //         let signed_token = access_token
    //             .sign(&active_keypair)
    //             .map_err(ErrorBoxed::from)?;

    //         Ok(signed_token)
    //     })
    //     .await?;

    // Ok(RefreshResponse {
    //     user: UserDto::from(&user),
    //     session_id: active_session.id().to_string(),
    //     signed_access_token,
    // })
}
