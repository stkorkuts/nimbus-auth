use std::path::PathBuf;

use crate::constants::{
    ACCESS_TOKEN_EXPIRATION_SECONDS_DEFAULT, SESSION_EXPIRATION_SECONDS_DEFAULT,
};

#[derive(Clone, Copy)]
pub struct SessionExpirationSeconds(pub u32);

#[derive(Clone, Copy)]
pub struct AccessTokenExpirationSeconds(pub u32);

pub struct AppConfigBuilder {
    server_addr: String,
    keypairs_store_path: PathBuf,
    session_expiration_seconds: Option<u32>,
    access_token_expiration_seconds: Option<u32>,
}

#[derive(Clone)]
pub struct AppConfig {
    server_addr: String,
    keypairs_store_path: PathBuf,
    session_expiration_seconds: SessionExpirationSeconds,
    access_token_expiration_seconds: AccessTokenExpirationSeconds,
}

pub struct AppConfigRequiredOptions {
    pub server_addr: String,
    pub keypairs_store_path: PathBuf,
}

impl AppConfigBuilder {
    pub fn new(
        AppConfigRequiredOptions {
            server_addr,
            keypairs_store_path,
        }: AppConfigRequiredOptions,
    ) -> Self {
        Self {
            server_addr,
            keypairs_store_path,
            session_expiration_seconds: None,
            access_token_expiration_seconds: None,
        }
    }

    pub fn with_session_expiration_seconds(&mut self, seconds: u32) -> &mut Self {
        self.session_expiration_seconds = Some(seconds);
        self
    }

    pub fn with_access_token_expiration_seconds(&mut self, seconds: u32) -> &mut Self {
        self.access_token_expiration_seconds = Some(seconds);
        self
    }

    pub fn build(self) -> AppConfig {
        AppConfig {
            server_addr: self.server_addr,
            keypairs_store_path: self.keypairs_store_path,
            session_expiration_seconds: SessionExpirationSeconds(
                self.session_expiration_seconds
                    .unwrap_or(SESSION_EXPIRATION_SECONDS_DEFAULT),
            ),
            access_token_expiration_seconds: AccessTokenExpirationSeconds(
                self.access_token_expiration_seconds
                    .unwrap_or(ACCESS_TOKEN_EXPIRATION_SECONDS_DEFAULT),
            ),
        }
    }
}

impl AppConfig {
    pub fn server_addr(&self) -> String {
        self.server_addr.clone()
    }

    pub fn keypairs_store_path(&self) -> PathBuf {
        self.keypairs_store_path.clone()
    }

    pub fn session_expiration_seconds(&self) -> SessionExpirationSeconds {
        self.session_expiration_seconds
    }

    pub fn access_token_expiration_seconds(&self) -> AccessTokenExpirationSeconds {
        self.access_token_expiration_seconds
    }
}
