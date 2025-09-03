use crate::constants::{
    ACCESS_TOKEN_EXPIRATION_SECONDS_DEFAULT, SESSION_EXPIRATION_SECONDS_DEFAULT,
};

pub struct AppConfigBuilder {
    server_addr: String,
    session_expiration_seconds: Option<u32>,
    access_token_expiration_seconds: Option<u32>,
}

#[derive(Clone, Copy)]
pub struct SessionExpirationSeconds(pub u32);

#[derive(Clone, Copy)]
pub struct AccessTokenExpirationSeconds(pub u32);

#[derive(Clone)]
pub struct AppConfig {
    server_addr: String,
    session_expiration_seconds: SessionExpirationSeconds,
    access_token_expiration_seconds: AccessTokenExpirationSeconds,
}

pub struct AppConfigRequiredOptions {
    pub server_addr: String,
}

impl AppConfigBuilder {
    pub fn new(AppConfigRequiredOptions { server_addr }: AppConfigRequiredOptions) -> Self {
        Self {
            server_addr,
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

    pub fn session_expiration_seconds(&self) -> SessionExpirationSeconds {
        self.session_expiration_seconds
    }

    pub fn access_token_expiration_seconds(&self) -> AccessTokenExpirationSeconds {
        self.access_token_expiration_seconds
    }
}
