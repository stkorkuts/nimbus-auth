use nimbus_auth_shared::config::AppConfig;

use std::sync::Arc;

use crate::{
    services::{session_repository::SessionRepository, user_repository::UserRepository},
    use_cases::{
        refresh::handle_refresh,
        signin::{errors::SignInError, handle_signin},
        signup::{errors::SignUpError, handle_signup},
    },
};

mod signup;
pub use signup::errors::*;
pub use signup::schema::*;

mod signin;
pub use signin::errors::*;
pub use signin::schema::*;

mod refresh;
pub use refresh::errors::*;
pub use refresh::schema::*;

#[derive(Clone)]
pub struct UseCases {
    session_repository: Arc<dyn SessionRepository>,
    user_repository: Arc<dyn UserRepository>,
}

impl UseCases {
    pub fn new(
        _app_config: &AppConfig,
        session_repository: Arc<dyn SessionRepository>,
        user_repository: Arc<dyn UserRepository>,
    ) -> UseCases {
        Self {
            session_repository,
            user_repository,
        }
    }

    pub async fn signup(&self, request: SignUpRequest) -> Result<SignUpResponse, SignUpError> {
        handle_signup(
            request,
            self.user_repository.clone(),
            self.session_repository.clone(),
        )
        .await
    }

    pub async fn signin(&self, request: SignInRequest) -> Result<SignInResponse, SignInError> {
        handle_signin(
            request,
            self.user_repository.clone(),
            self.session_repository.clone(),
        )
        .await
    }

    pub async fn refresh(&self, request: RefreshRequest) -> Result<RefreshResponse, RefreshError> {
        handle_refresh(
            request,
            self.user_repository.clone(),
            self.session_repository.clone(),
        )
        .await
    }

    pub async fn get_public_key(&self) {
        todo!()
    }
}
