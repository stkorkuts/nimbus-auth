use std::sync::Arc;

use nimbus_auth_shared::config::AppConfig;

use crate::{
    services::{session_repository::SessionRepository, user_repository::UserRepository},
    use_cases::{SignUpRequest, SignUpResponse, signup::errors::SignUpError},
};

pub mod errors;
pub mod schema;

pub async fn handle_signup(
    SignUpRequest {}: SignUpRequest,
    user_repository: Arc<dyn UserRepository>,
    session_repository: Arc<dyn SessionRepository>,
) -> Result<SignUpResponse, SignUpError> {
    todo!()
}
