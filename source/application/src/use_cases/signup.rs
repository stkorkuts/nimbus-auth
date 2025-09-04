use std::{path::PathBuf, sync::Arc};

use nimbus_auth_shared::config::{
    AccessTokenExpirationSeconds, AppConfig, SessionExpirationSeconds,
};

use crate::{
    services::{
        keypair_repository::{self, KeyPairRepository},
        session_repository::SessionRepository,
        user_repository::UserRepository,
    },
    use_cases::{SignUpRequest, SignUpResponse, signup::errors::SignUpError},
};

pub mod errors;
pub mod schema;

pub async fn handle_signup(
    SignUpRequest {}: SignUpRequest,
    user_repository: Arc<dyn UserRepository>,
    session_repository: Arc<dyn SessionRepository>,
    keypair_repository: Arc<dyn KeyPairRepository>,
    session_exp_seconds: SessionExpirationSeconds,
    access_token_exp_seconds: AccessTokenExpirationSeconds,
) -> Result<SignUpResponse, SignUpError> {
    todo!()
}
