use std::sync::Arc;

use nimbus_auth_domain::entities::{
    Entity,
    session::{InitializedSession, Session, specifications::NewSessionSpecification},
    user::{
        User,
        specifications::NewUserSpecification,
        value_objects::{name::UserName, password::Password},
    },
};
use nimbus_auth_shared::config::{AccessTokenExpirationSeconds, SessionExpirationSeconds};

use crate::{
    services::{
        keypair_repository::KeyPairRepository,
        session_repository::{self, SessionRepository},
        time_service::TimeService,
        transactions::{TransactionIsolationLevel, TransactonBlockTarget},
        user_repository::UserRepository,
    },
    use_cases::{SignUpRequest, SignUpResponse, signup::errors::SignUpError},
};

pub mod errors;
pub mod schema;

pub async fn handle_signup<'a>(
    SignUpRequest {
        user_name,
        password,
    }: SignUpRequest<'a>,
    user_repository: Arc<dyn UserRepository>,
    session_repository: Arc<dyn SessionRepository>,
    keypair_repository: Arc<dyn KeyPairRepository>,
    time_service: Arc<dyn TimeService>,
    session_exp_seconds: SessionExpirationSeconds,
    access_token_exp_seconds: AccessTokenExpirationSeconds,
) -> Result<SignUpResponse, SignUpError> {
    let user_name = UserName::from(user_name)?;

    let existing_user = user_repository.get_by_name(&user_name, None).await?;

    let password = Password::from(password)?;

    if let Some(user) = existing_user {
        return Err(SignUpError::UserAlreadyExists {
            user_name: user.name().to_string(),
        });
    }

    let current_time = time_service.get_current_time().await?;

    let active_keypair = keypair_repository
        .get_active(None)
        .await?
        .ok_or(SignUpError::ActiveKeyPairNotFound)?;

    let user = User::new(NewUserSpecification {
        user_name,
        password,
    });

    let session = Session::new(NewSessionSpecification {
        user_id: user.id().clone(),
        current_time,
        expiration_seconds: session_exp_seconds,
    });

    let mut user_repo_transaction = user_repository
        .start_transaction(
            TransactionIsolationLevel::Default,
            TransactonBlockTarget::Default,
        )
        .await?;

    let signed_token = user_repo_transaction
        .run(async move |inner_user_repo_transacton| {
            user_repository
                .save(&user, Some(inner_user_repo_transacton.clone()))
                .await?;

            let mut session_repo_transaction = session_repository
                .start_transaction(
                    TransactionIsolationLevel::Default,
                    TransactonBlockTarget::Default,
                )
                .await?;

            Ok(session_repo_transaction
                .run(async move |inner_session_repo_transaction| {
                    session_repository
                        .save(
                            &InitializedSession::from(session),
                            Some(inner_session_repo_transaction),
                        )
                        .await?;

                    let access_token =
                        session.generate_access_token(current_time, access_token_exp_seconds);
                    let signed_token = access_token.sign(&active_keypair)?;

                    Ok(signed_token)
                })
                .await?)
        })
        .await?;
    todo!()
}
