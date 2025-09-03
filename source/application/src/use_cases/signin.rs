use std::sync::Arc;

use nimbus_auth_shared::config::AppConfig;

use crate::{
    services::{session_repository::SessionRepository, user_repository::UserRepository},
    use_cases::{SignInRequest, SignInResponse, signin::errors::SignInError},
};

pub mod errors;
pub mod schema;

pub async fn handle_signin(
    SignInRequest {}: SignInRequest,
    user_repository: Arc<dyn UserRepository>,
    session_repository: Arc<dyn SessionRepository>,
) -> Result<SignInResponse, SignInError> {
    todo!()
}
