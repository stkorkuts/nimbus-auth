use nimbus_auth_shared::config::{
    AccessTokenExpirationSeconds, AppConfig, SessionExpirationSeconds,
};

use std::{path::PathBuf, sync::Arc};

use crate::{
    services::{session_repository::SessionRepository, user_repository::UserRepository},
    use_cases::{
        get_public_key::handle_get_public_key,
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

mod get_public_key;
pub use get_public_key::errors::*;
pub use get_public_key::schema::*;

#[derive(Clone)]
pub struct UseCases {
    config: UseCasesConfig,
    services: UseCasesServices,
}

#[derive(Clone)]
struct UseCasesConfig {
    pub public_key_path: PathBuf,
    pub private_key_path: PathBuf,
    pub session_expiration_seconds: SessionExpirationSeconds,
    pub access_token_expiration_seconds: AccessTokenExpirationSeconds,
}

#[derive(Clone)]
struct UseCasesServices {
    pub session_repository: Arc<dyn SessionRepository>,
    pub user_repository: Arc<dyn UserRepository>,
}

impl UseCases {
    pub fn new(
        app_config: &AppConfig,
        session_repository: Arc<dyn SessionRepository>,
        user_repository: Arc<dyn UserRepository>,
    ) -> UseCases {
        Self {
            config: UseCasesConfig {
                public_key_path: app_config.public_key_path(),
                private_key_path: app_config.private_key_path(),
                session_expiration_seconds: app_config.session_expiration_seconds(),
                access_token_expiration_seconds: app_config.access_token_expiration_seconds(),
            },
            services: UseCasesServices {
                session_repository,
                user_repository,
            },
        }
    }

    pub async fn signup(&self, request: SignUpRequest) -> Result<SignUpResponse, SignUpError> {
        handle_signup(
            request,
            self.services.user_repository.clone(),
            self.services.session_repository.clone(),
            &self.config.private_key_path,
            self.config.session_expiration_seconds,
            self.config.access_token_expiration_seconds,
        )
        .await
    }

    pub async fn signin(&self, request: SignInRequest) -> Result<SignInResponse, SignInError> {
        handle_signin(
            request,
            self.services.user_repository.clone(),
            self.services.session_repository.clone(),
            &self.config.private_key_path,
            self.config.session_expiration_seconds,
            self.config.access_token_expiration_seconds,
        )
        .await
    }

    pub async fn refresh(&self, request: RefreshRequest) -> Result<RefreshResponse, RefreshError> {
        handle_refresh(
            request,
            self.services.user_repository.clone(),
            self.services.session_repository.clone(),
            &self.config.private_key_path,
            self.config.session_expiration_seconds,
            self.config.access_token_expiration_seconds,
        )
        .await
    }

    pub async fn get_public_key(
        &self,
        request: GetPublicKeyRequest,
    ) -> Result<GetPublicKeyResponse, GetPublicKeyError> {
        handle_get_public_key(request, &self.config.public_key_path).await
    }
}
