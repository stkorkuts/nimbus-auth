use std::{path::PathBuf, sync::Arc};

use nimbus_auth_shared::config::{
    AccessTokenExpirationSeconds, AppConfig, SessionExpirationSeconds,
};

use crate::{
    services::{session_repository::SessionRepository, user_repository::UserRepository},
    use_cases::{RefreshRequest, RefreshResponse, refresh::errors::RefreshError},
};

pub mod errors;
pub mod schema;

pub async fn handle_refresh(
    RefreshRequest {}: RefreshRequest,
    user_repository: Arc<dyn UserRepository>,
    session_repository: Arc<dyn SessionRepository>,
    private_key_path: &PathBuf,
    session_exp_seconds: SessionExpirationSeconds,
    access_token_exp_seconds: AccessTokenExpirationSeconds,
) -> Result<RefreshResponse, RefreshError> {
    todo!()
}
