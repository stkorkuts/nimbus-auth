use std::{borrow::Cow, sync::Arc};

use nimbus_auth_domain::entities::{
    Entity,
    session::{SomeSession, specifications::NewSessionSpecification},
    user::value_objects::{password::Password, user_name::UserName},
};
use nimbus_auth_shared::types::{AccessTokenExpirationSeconds, SessionExpirationSeconds};
use zeroize::Zeroizing;

use crate::{
    services::{
        keypair_repository::KeyPairRepository, session_repository::SessionRepository,
        time_service::TimeService, user_repository::UserRepository,
    },
    use_cases::{
        UserClaimsDto,
        dtos::{access_token::AccessTokenDto, session::SessionDto},
        signin::{
            errors::SignInError,
            schema::{SignInRequest, SignInResponse},
        },
    },
};

pub mod errors;
pub mod schema;

pub async fn handle_signin<'a>(
    SignInRequest {
        user_name,
        password,
    }: SignInRequest<'a>,
    user_repository: Arc<dyn UserRepository>,
    session_repository: Arc<dyn SessionRepository>,
    keypair_repository: Arc<dyn KeyPairRepository>,
    time_service: Arc<dyn TimeService>,
    session_exp_seconds: SessionExpirationSeconds,
    access_token_exp_seconds: AccessTokenExpirationSeconds,
) -> Result<SignInResponse, SignInError> {
    let user_name = UserName::from(user_name)?;

    let user =
        user_repository
            .get_by_name(&user_name)
            .await?
            .ok_or(SignInError::UserIsNotFound {
                user_name: user_name.to_string(),
            })?;

    let password = Password::from(password)?;

    if !user.password_hash().verify(&password) {
        return Err(SignInError::PasswordDoesNotMatchWithHash);
    }

    let active_keypair = keypair_repository
        .get_active()
        .await?
        .ok_or(SignInError::ActiveKeyPairNotFound)?;

    let session = SomeSession::new(NewSessionSpecification {
        user_claims: user.claims().clone(),
        current_time: time_service.get_current_time().await?,
        expiration_seconds: session_exp_seconds,
    });

    let transactional_session_repository = session_repository.start_transaction().await?;

    let (transactional_session_repository, _) = transactional_session_repository
        .save(SomeSession::Active(Cow::Borrowed(&session)))
        .await?;

    let access_token = &session.generate_access_token(
        time_service.get_current_time().await?,
        access_token_exp_seconds,
    );
    let signed_access_token = access_token.sign(&active_keypair)?;

    transactional_session_repository.commit().await?;

    let user_dto = UserClaimsDto::from(user.claims());

    let session_dto = SessionDto {
        session_id: Zeroizing::new(session.id().to_string()),
        session_expires_at_unix_timestamp: session.expires_at().unix_timestamp(),
    };

    let access_token_dto = AccessTokenDto {
        signed_access_token,
        signed_access_token_expires_at_unix_timestamp: access_token.expires_at().unix_timestamp(),
    };

    Ok(SignInResponse {
        user: user_dto,
        session: session_dto,
        access_token: access_token_dto,
    })
}
