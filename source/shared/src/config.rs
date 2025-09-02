use crate::constants::{
    ACCESS_TOKEN_EXPIRATION_SECONDS_DEFAULT, SESSION_EXPIRATION_SECONDS_DEFAULT,
};

pub struct AppConfigBuilder {
    session_expiration_seconds: Option<u32>,
    access_token_expiration_seconds: Option<u32>,
}

pub struct AppConfig {
    session_expiration_seconds: u32,
    access_token_expiration_seconds: u32,
}

impl AppConfigBuilder {
    pub fn new() -> Self {
        Self {
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
            session_expiration_seconds: self
                .session_expiration_seconds
                .unwrap_or(SESSION_EXPIRATION_SECONDS_DEFAULT),
            access_token_expiration_seconds: self
                .access_token_expiration_seconds
                .unwrap_or(ACCESS_TOKEN_EXPIRATION_SECONDS_DEFAULT),
        }
    }
}

impl AppConfig {
    fn session_expiration_seconds(&self) -> u32 {
        self.session_expiration_seconds
    }

    fn access_token_expiration_seconds(&self) -> u32 {
        self.access_token_expiration_seconds
    }
}
