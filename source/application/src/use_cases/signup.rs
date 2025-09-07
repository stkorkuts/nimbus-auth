use std::sync::Arc;

use nimbus_auth_domain::entities::user::{
    User, specifications::NewUserSpecification, value_objects::name::UserName,
};
use nimbus_auth_shared::config::{AccessTokenExpirationSeconds, SessionExpirationSeconds};

use crate::{
    services::{
        keypair_repository::KeyPairRepository, session_repository::SessionRepository,
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
        e2e_key_hash,
        encrypted_master_key,
    }: SignUpRequest<'a>,
    user_repository: Arc<dyn UserRepository>,
    session_repository: Arc<dyn SessionRepository>,
    keypair_repository: Arc<dyn KeyPairRepository>,
    session_exp_seconds: SessionExpirationSeconds,
    access_token_exp_seconds: AccessTokenExpirationSeconds,
) -> Result<SignUpResponse, SignUpError> {
    let user_name = UserName::from(user_name)?;

    let existing_user = user_repository.get_by_name(user_name, None).await?;

    if let Some(user) = existing_user {
        return Err(SignUpError::UserAlreadyExists {
            user_name: user.name().to_string(),
        });
    }

    let user = User::new(NewUserSpecification {
        user_name,
        password,
        e2e_key_hash,
        encrypted_master_key,
    })?;

    todo!()
}
