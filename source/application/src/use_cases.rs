use nimbus_auth_shared::types::{AccessTokenExpirationSeconds, SessionExpirationSeconds};

use std::sync::Arc;

use crate::{
    services::{
        keypair_repository::KeyPairRepository, random_service::RandomService,
        session_repository::SessionRepository, time_service::TimeService,
        user_repository::UserRepository,
    },
    use_cases::{
        get_public_key::handle_get_public_key,
        refresh::handle_refresh,
        rotate_keypairs::handle_rotate_keypairs,
        signin::{errors::SignInError, handle_signin},
        signup::{errors::SignUpError, handle_signup},
    },
};

mod dtos;
pub use dtos::user::*;

mod signup;
pub use signup::schema::*;

mod signin;
pub use signin::schema::*;

mod refresh;
pub use refresh::errors::*;
pub use refresh::schema::*;

mod get_public_key;
pub use get_public_key::errors::*;
pub use get_public_key::schema::*;

mod rotate_keypairs;
pub use rotate_keypairs::errors::*;
pub use rotate_keypairs::schema::*;

#[derive(Clone)]
pub struct UseCases {
    config: UseCasesConfig,
    services: UseCasesServices,
}

#[derive(Clone)]
pub struct UseCasesConfig {
    pub session_expiration_seconds: SessionExpirationSeconds,
    pub access_token_expiration_seconds: AccessTokenExpirationSeconds,
}

#[derive(Clone)]
pub struct UseCasesServices {
    pub session_repository: Arc<dyn SessionRepository>,
    pub user_repository: Arc<dyn UserRepository>,
    pub keypair_repository: Arc<dyn KeyPairRepository>,
    pub time_service: Arc<dyn TimeService>,
    pub random_service: Arc<dyn RandomService>,
}

impl<'a> UseCases {
    pub fn new(config: UseCasesConfig, services: UseCasesServices) -> UseCases {
        Self { config, services }
    }

    pub async fn rotate_keypairs(
        &self,
        request: RotateKeyPairsRequest,
    ) -> Result<RotateKeyPairsResponse, RotateKeyPairsError> {
        handle_rotate_keypairs(
            request,
            self.services.keypair_repository.clone(),
            self.services.time_service.clone(),
            self.services.random_service.clone(),
            self.config.access_token_expiration_seconds,
        )
        .await
    }

    pub async fn signup(&self, request: SignUpRequest<'a>) -> Result<SignUpResponse, SignUpError> {
        handle_signup(
            request,
            self.services.user_repository.clone(),
            self.services.session_repository.clone(),
            self.services.keypair_repository.clone(),
            self.services.time_service.clone(),
            self.config.session_expiration_seconds,
            self.config.access_token_expiration_seconds,
        )
        .await
    }

    pub async fn signin(&self, request: SignInRequest<'a>) -> Result<SignInResponse, SignInError> {
        handle_signin(
            request,
            self.services.user_repository.clone(),
            self.services.session_repository.clone(),
            self.services.keypair_repository.clone(),
            self.services.time_service.clone(),
            self.config.session_expiration_seconds,
            self.config.access_token_expiration_seconds,
        )
        .await
    }

    pub async fn get_public_key(
        &self,
        request: GetPublicKeyRequest<'a>,
    ) -> Result<GetPublicKeyResponse, GetPublicKeyError> {
        handle_get_public_key(request, self.services.keypair_repository.clone()).await
    }

    pub async fn refresh(&self, request: RefreshRequest) -> Result<RefreshResponse, RefreshError> {
        handle_refresh(
            request,
            self.services.user_repository.clone(),
            self.services.session_repository.clone(),
            self.services.keypair_repository.clone(),
            self.services.time_service.clone(),
            self.config.session_expiration_seconds,
            self.config.access_token_expiration_seconds,
        )
        .await
    }
}
