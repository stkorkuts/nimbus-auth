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
    public_key_path: PathBuf,
    private_key_path: PathBuf,
    session_expiration_seconds: Option<u32>,
    access_token_expiration_seconds: Option<u32>,
}

#[derive(Clone)]
pub struct AppConfig {
    server_addr: String,
    public_key_path: PathBuf,
    private_key_path: PathBuf,
    session_expiration_seconds: SessionExpirationSeconds,
    access_token_expiration_seconds: AccessTokenExpirationSeconds,
}

pub struct AppConfigRequiredOptions {
    pub server_addr: String,
    pub public_key_path: PathBuf,
    pub private_key_path: PathBuf,
}

impl AppConfigBuilder {
    pub fn new(
        AppConfigRequiredOptions {
            server_addr,
            public_key_path,
            private_key_path,
        }: AppConfigRequiredOptions,
    ) -> Self {
        Self {
            server_addr,
            public_key_path,
            private_key_path,
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
            private_key_path: self.private_key_path,
            public_key_path: self.public_key_path,
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

    pub fn public_key_path(&self) -> PathBuf {
        self.public_key_path.clone()
    }

    pub fn private_key_path(&self) -> PathBuf {
        self.private_key_path.clone()
    }

    pub fn session_expiration_seconds(&self) -> SessionExpirationSeconds {
        self.session_expiration_seconds
    }

    pub fn access_token_expiration_seconds(&self) -> AccessTokenExpirationSeconds {
        self.access_token_expiration_seconds
    }
}
