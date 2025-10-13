use std::path::PathBuf;

use crate::{
    constants::{
        ACCESS_TOKEN_EXPIRATION_SECONDS_DEFAULT, POSTGRESDB_MAX_CONNECTIONS_DEFAULT,
        SESSION_EXPIRATION_SECONDS_DEFAULT,
    },
    types::{AccessTokenExpirationSeconds, PostgresDbMaxConnections, SessionExpirationSeconds},
};

pub struct AppConfigBuilder {
    server_addr: String,
    keypairs_store_path: PathBuf,
    postgres_db_url: String,
    session_expiration_seconds: Option<usize>,
    access_token_expiration_seconds: Option<usize>,
    postgres_db_max_connections: Option<usize>,
    use_hsts: Option<()>,
    cors_origins: Option<Vec<String>>,
}

#[derive(Clone)]
pub struct AppConfig {
    server_addr: String,
    keypairs_store_path: PathBuf,
    postgres_db_url: String,
    session_expiration_seconds: SessionExpirationSeconds,
    access_token_expiration_seconds: AccessTokenExpirationSeconds,
    postgres_db_max_connections: PostgresDbMaxConnections,
    use_hsts: bool,
    cors_origins: Option<Vec<String>>,
}

pub struct AppConfigRequiredOptions {
    pub server_addr: String,
    pub keypairs_store_path: PathBuf,
    pub postgres_db_url: String,
}

impl AppConfigBuilder {
    pub fn new(
        AppConfigRequiredOptions {
            server_addr,
            keypairs_store_path,
            postgres_db_url,
        }: AppConfigRequiredOptions,
    ) -> Self {
        Self {
            server_addr,
            keypairs_store_path,
            postgres_db_url,
            session_expiration_seconds: None,
            access_token_expiration_seconds: None,
            postgres_db_max_connections: None,
            use_hsts: None,
            cors_origins: None,
        }
    }

    pub fn with_session_expiration_seconds(&mut self, seconds: usize) -> &mut Self {
        self.session_expiration_seconds = Some(seconds);
        self
    }

    pub fn with_access_token_expiration_seconds(&mut self, seconds: usize) -> &mut Self {
        self.access_token_expiration_seconds = Some(seconds);
        self
    }

    pub fn with_postgres_db_max_connections(&mut self, connections: usize) -> &mut Self {
        self.postgres_db_max_connections = Some(connections);
        self
    }

    pub fn with_hsts(&mut self) -> &mut Self {
        self.use_hsts = Some(());
        self
    }

    pub fn with_cors_origins(&mut self, origins: Vec<String>) -> &mut Self {
        self.cors_origins = Some(origins);
        self
    }

    pub fn build(self) -> AppConfig {
        AppConfig {
            server_addr: self.server_addr,
            keypairs_store_path: self.keypairs_store_path,
            postgres_db_url: self.postgres_db_url,
            session_expiration_seconds: SessionExpirationSeconds(
                self.session_expiration_seconds
                    .unwrap_or(SESSION_EXPIRATION_SECONDS_DEFAULT),
            ),
            access_token_expiration_seconds: AccessTokenExpirationSeconds(
                self.access_token_expiration_seconds
                    .unwrap_or(ACCESS_TOKEN_EXPIRATION_SECONDS_DEFAULT),
            ),
            postgres_db_max_connections: PostgresDbMaxConnections(
                self.postgres_db_max_connections
                    .unwrap_or(POSTGRESDB_MAX_CONNECTIONS_DEFAULT),
            ),
            use_hsts: self.use_hsts.is_some(),
            cors_origins: self.cors_origins,
        }
    }
}

impl AppConfig {
    pub fn server_addr(&self) -> &str {
        &self.server_addr
    }

    pub fn keypairs_store_path(&self) -> &PathBuf {
        &self.keypairs_store_path
    }

    pub fn postgres_db_url(&self) -> &str {
        &self.postgres_db_url
    }

    pub fn session_expiration_seconds(&self) -> SessionExpirationSeconds {
        self.session_expiration_seconds
    }

    pub fn access_token_expiration_seconds(&self) -> AccessTokenExpirationSeconds {
        self.access_token_expiration_seconds
    }

    pub fn postgres_db_max_connections(&self) -> PostgresDbMaxConnections {
        self.postgres_db_max_connections
    }

    pub fn use_hsts(&self) -> bool {
        self.use_hsts
    }

    pub fn cors_origins(&self) -> &Option<Vec<String>> {
        &self.cors_origins
    }
}
