use std::sync::Arc;

use nimbus_auth_domain::{
    entities::{
        Entity,
        session::{InitializedSessionRef, Session, specifications::NewSessionSpecification},
        user::value_objects::{name::UserName, password::Password},
    },
    value_objects::identifier::IdentifierOfType,
};
use nimbus_auth_shared::{
    config::{AccessTokenExpirationSeconds, SessionExpirationSeconds},
    errors::ErrorBoxed,
};

use crate::{
    services::{
        keypair_repository::KeyPairRepository,
        session_repository::SessionRepository,
        time_service::TimeService,
        transactions::{TransactionIsolationLevel, TransactonBlockTarget},
        user_repository::UserRepository,
    },
    use_cases::{
        UserDto,
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

    let user = Arc::new(user_repository.get_by_name(&user_name, None).await?.ok_or(
        SignInError::UserIsNotFound {
            user_name: user_name.to_string(),
        },
    )?);

    let password = Password::from(password)?;

    if !user.password_hash().verify(password) {
        return Err(SignInError::PasswordDoesNotMatchWithHash);
    }

    let current_time = time_service.get_current_time().await?;

    let active_keypair = keypair_repository
        .get_active(None)
        .await?
        .ok_or(SignInError::ActiveKeyPairNotFound)?;

    let session = Arc::new(Session::new(NewSessionSpecification {
        user_id: user.id().clone(),
        current_time,
        expiration_seconds: session_exp_seconds,
    }));

    let session_for_transaction = session.clone();

    let mut session_repo_transaction = session_repository
        .start_transaction(
            TransactionIsolationLevel::Default,
            TransactonBlockTarget::Default,
        )
        .await?;

    let signed_token = session_repo_transaction
        .run(async move |inner_session_repo_transaction| {
            session_repository
                .save(
                    InitializedSessionRef::from(session_for_transaction.clone().as_ref()),
                    Some(inner_session_repo_transaction),
                )
                .await?;

            let access_token = session_for_transaction
                .clone()
                .generate_access_token(current_time, access_token_exp_seconds);
            let signed_token = access_token
                .sign(&active_keypair)
                .map_err(|err| ErrorBoxed::from(err))?;

            Ok(signed_token)
        })
        .await?;

    Ok(SignInResponse {
        user: UserDto::from(user.as_ref()),
        session_id: session.id().value().to_string(),
        signed_access_token: signed_token,
    })
}
