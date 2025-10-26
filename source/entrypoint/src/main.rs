use std::{env, sync::Arc};

use nimbus_auth_application::use_cases::{UseCases, UseCasesConfig, UseCasesServices};
use nimbus_auth_infrastructure::{
    postgres_db::PostgresDatabase,
    services_implementations::{
        filesystem_inmemory_cached_keypair_repository::FileSystemInMemoryCachedKeyPairRepository,
        os_random_service::OsRandomService, os_time_service::OsTimeService,
        postgres_session_repository::PostgresSessionRepository,
        postgres_user_repository::PostgresUserRepository,
    },
    web_api::WebApi,
};
use nimbus_auth_shared::{
    config::{AppConfig, AppConfigBuilder, AppConfigRequiredOptions},
    constants::{
        ACCESS_TOKEN_EXPIRATION_SECONDS_ENV_VAR_NAME, CORS_ORIGINS_COMMA_SEPARATED_ENV_VAR_NAME,
        KEYPAIRS_STORE_PATH_ENV_VAR_NAME, POSTGRESDB_MAX_CONNECTIONS_ENV_VAR_NAME,
        POSTGRESQL_URL_ENV_VAR_NAME, SERVER_ADDR_ENV_VAR_NAME,
        SESSION_EXPIRATION_SECONDS_ENV_VAR_NAME, USE_HSTS_ENV_VAR_NAME,
    },
    errors::{ErrorBoxed, ErrorContextExt},
};
use tokio::io;
#[cfg(unix)]
use tokio::signal::unix::{SignalKind, signal};
use tokio::sync::oneshot;
use tracing::subscriber;
use tracing_subscriber::FmtSubscriber;

use crate::errors::EntryPointError;

mod errors;

#[tokio::main]
async fn main() -> Result<(), EntryPointError> {
    let config = get_config_from_env()?;

    configure_tracing(&config)?;

    let use_cases = build_use_cases(&config).await?;

    let (shutdown_signal_sender, shutdown_signal_receiver) = oneshot::channel();
    let ctrl_c = tokio::signal::ctrl_c();
    #[cfg(unix)]
    let sigterm = async {
        let mut stream = signal(SignalKind::terminate())?;
        stream.recv().await;
        Ok::<(), io::Error>(())
    };
    #[cfg(not(unix))]
    let sigterm = std::future::pending::<()>();

    tokio::select! {
        res = WebApi::serve(&config, use_cases, shutdown_signal_receiver) => res?,
        res = ctrl_c => res?,
        res = sigterm => res?
    }

    if let Err(_) = shutdown_signal_sender.send(()) {
        return Err(EntryPointError::ShutdownSignalSending);
    }

    Ok(())
}

fn get_config_from_env() -> Result<AppConfig, ErrorBoxed> {
    dotenvy::dotenv()?;

    let mut config_builder = AppConfigBuilder::new(AppConfigRequiredOptions {
        server_addr: env::var(SERVER_ADDR_ENV_VAR_NAME).map_err(|err| {
            err.with_context(format!(
                "env variable ({SERVER_ADDR_ENV_VAR_NAME}) is required and not presented"
            ))
        })?,
        keypairs_store_path: env::var(KEYPAIRS_STORE_PATH_ENV_VAR_NAME)
            .map_err(|err| {
                err.with_context(format!(
                    "env variable ({KEYPAIRS_STORE_PATH_ENV_VAR_NAME}) is required and not presented"
                ))
            })?
            .parse()?,
        postgres_db_url: env::var(POSTGRESQL_URL_ENV_VAR_NAME)
            .map_err(|err| {
                err.with_context(format!(
                    "env variable ({POSTGRESQL_URL_ENV_VAR_NAME}) is required and not presented"
                ))
            })?
            .parse()?,
    });

    if let Ok(value) = env::var(SESSION_EXPIRATION_SECONDS_ENV_VAR_NAME) {
        let parsed = value.parse().map_err(|err: std::num::ParseIntError| {
            err.with_context(format!(
                "env variable ({SESSION_EXPIRATION_SECONDS_ENV_VAR_NAME}) has wrong format, it should be integer"
            ))
        })?;
        config_builder.with_session_expiration_seconds(parsed);
    };

    if let Ok(value) = env::var(ACCESS_TOKEN_EXPIRATION_SECONDS_ENV_VAR_NAME) {
        let parsed = value.parse().map_err(|err: std::num::ParseIntError| {
            err.with_context(format!(
                "env variable ({ACCESS_TOKEN_EXPIRATION_SECONDS_ENV_VAR_NAME}) has wrong format, it should be integer"
            ))
        })?;
        config_builder.with_access_token_expiration_seconds(parsed);
    }

    if let Ok(value) = env::var(POSTGRESDB_MAX_CONNECTIONS_ENV_VAR_NAME) {
        let parsed = value.parse().map_err(|err: std::num::ParseIntError| {
            err.with_context(format!(
                "env variable ({POSTGRESDB_MAX_CONNECTIONS_ENV_VAR_NAME}) has wrong format, it should be integer"
            ))
        })?;
        config_builder.with_postgres_db_max_connections(parsed);
    }

    if let Ok(value) = env::var(USE_HSTS_ENV_VAR_NAME)
        && value.parse::<bool>().map_err(|err| {
            err.with_context(format!(
                "env variable ({USE_HSTS_ENV_VAR_NAME}) has wrong format, it should be `true` or `false`"
            ))
        })?
    {
        config_builder.with_hsts();
    }

    if let Ok(value) = env::var(CORS_ORIGINS_COMMA_SEPARATED_ENV_VAR_NAME) {
        config_builder.with_cors_origins_comma_separated(&value);
    }

    Ok(config_builder.build()?)
}

fn configure_tracing(_: &AppConfig) -> Result<(), ErrorBoxed> {
    let subscriber = FmtSubscriber::builder().finish();

    subscriber::set_global_default(subscriber)?;

    Ok(())
}

async fn build_use_cases(app_config: &AppConfig) -> Result<UseCases, ErrorBoxed> {
    let use_cases_config = UseCasesConfig {
        session_expiration_seconds: app_config.session_expiration_seconds(),
        access_token_expiration_seconds: app_config.access_token_expiration_seconds(),
    };

    let postgres_db = Arc::new(PostgresDatabase::new(app_config).await?);

    let session_repository = Arc::new(PostgresSessionRepository::new(postgres_db.clone()));
    let user_repository = Arc::new(PostgresUserRepository::new(postgres_db.clone()));
    let keypair_repository = Arc::new(
        FileSystemInMemoryCachedKeyPairRepository::init(app_config.keypairs_store_path()).await?,
    );
    let time_service = Arc::new(OsTimeService::new());
    let random_service = Arc::new(OsRandomService::new());

    let use_cases_services = UseCasesServices {
        session_repository,
        user_repository,
        keypair_repository,
        time_service,
        random_service,
    };

    Ok(UseCases::new(use_cases_config, use_cases_services))
}
