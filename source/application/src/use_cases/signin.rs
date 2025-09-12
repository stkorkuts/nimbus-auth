use std::sync::Arc;

use nimbus_auth_domain::entities::user::value_objects::{name::UserName, password::Password};
use nimbus_auth_shared::config::{AccessTokenExpirationSeconds, SessionExpirationSeconds};

use crate::{
    services::{
        keypair_repository::KeyPairRepository, session_repository::SessionRepository,
        time_service::TimeService, user_repository::UserRepository,
    },
    use_cases::signin::{
        errors::SignInError,
        schema::{SignInRequest, SignInResponse},
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

    todo!()
}
