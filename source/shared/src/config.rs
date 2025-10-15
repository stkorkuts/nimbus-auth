use std::path::PathBuf;

use url::{ParseError, Url};

use crate::{
    constants::{
        ACCESS_TOKEN_EXPIRATION_SECONDS_DEFAULT, CORS_ORIGINS_COMMA_SEPARATED_DEFAULT,
        POSTGRESDB_MAX_CONNECTIONS_DEFAULT, SESSION_EXPIRATION_SECONDS_DEFAULT, USE_HSTS_DEFAULT,
    },
    errors::AppConfigBuilderError,
    types::{AccessTokenExpirationSeconds, PostgresDbMaxConnections, SessionExpirationSeconds},
};

pub struct AppConfigBuilder {
    server_addr: String,
    keypairs_store_path: PathBuf,
    postgres_db_url: String,
    session_expiration_seconds: usize,
    access_token_expiration_seconds: usize,
    postgres_db_max_connections: usize,
    use_hsts: bool,
    cors_origins_comma_separated: String,
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
    cors_origins: Vec<String>,
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
            session_expiration_seconds: SESSION_EXPIRATION_SECONDS_DEFAULT,
            access_token_expiration_seconds: ACCESS_TOKEN_EXPIRATION_SECONDS_DEFAULT,
            postgres_db_max_connections: POSTGRESDB_MAX_CONNECTIONS_DEFAULT,
            use_hsts: USE_HSTS_DEFAULT,
            cors_origins_comma_separated: CORS_ORIGINS_COMMA_SEPARATED_DEFAULT.to_string(),
        }
    }

    pub fn with_session_expiration_seconds(&mut self, seconds: usize) -> &mut Self {
        self.session_expiration_seconds = seconds;
        self
    }

    pub fn with_access_token_expiration_seconds(&mut self, seconds: usize) -> &mut Self {
        self.access_token_expiration_seconds = seconds;
        self
    }

    pub fn with_postgres_db_max_connections(&mut self, connections: usize) -> &mut Self {
        self.postgres_db_max_connections = connections;
        self
    }

    pub fn with_hsts(&mut self) -> &mut Self {
        self.use_hsts = true;
        self
    }

    pub fn with_cors_origins_comma_separated(
        &mut self,
        origins_comma_separated: &str,
    ) -> &mut Self {
        self.cors_origins_comma_separated = origins_comma_separated.to_string();
        self
    }

    pub fn build(self) -> Result<AppConfig, AppConfigBuilderError> {
        Ok(AppConfig {
            server_addr: self.server_addr,
            keypairs_store_path: self.keypairs_store_path,
            postgres_db_url: self.postgres_db_url,
            session_expiration_seconds: SessionExpirationSeconds(self.session_expiration_seconds),
            access_token_expiration_seconds: AccessTokenExpirationSeconds(
                self.access_token_expiration_seconds,
            ),
            postgres_db_max_connections: PostgresDbMaxConnections(self.postgres_db_max_connections),
            use_hsts: self.use_hsts,
            cors_origins: Self::parse_cors_origins_comma_separated(
                &self.cors_origins_comma_separated,
            )?,
        })
    }

    fn parse_cors_origins_comma_separated(
        cors_origins_comma_separated: &str,
    ) -> Result<Vec<String>, ParseError> {
        cors_origins_comma_separated
            .split(",")
            .filter(|origin| !origin.trim().is_empty())
            .map(|origin| Url::parse(origin.trim()).map(|url| url.to_string()))
            .collect()
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

    pub fn cors_origins(&self) -> &Vec<String> {
        &self.cors_origins
    }
}
