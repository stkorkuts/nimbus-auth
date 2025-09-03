use std::sync::Arc;

use nimbus_auth_shared::config::AppConfig;

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
) -> Result<RefreshResponse, RefreshError> {
    todo!()
}
