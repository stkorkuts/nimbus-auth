use std::{env, error::Error, sync::Arc};

use nimbus_auth_application::{
    services::user_repository,
    use_cases::{UseCases, UseCasesConfig, UseCasesServices},
};
use nimbus_auth_infrastructure::{
    axum_api::WebApi,
    postgres_db::PostgresDatabase,
    services_implementations::{
        filesystem_keypair_repository::FileSystemKeyPairRepository,
        os_random_service::OsRandomService, os_time_service::OsTimeService,
        postgres_session_repository::PostgresSessionRepository,
        postgres_user_repository::PostgresUserRepository,
    },
};
use nimbus_auth_shared::{
    config::{AppConfig, AppConfigBuilder, AppConfigRequiredOptions},
    constants::{
        ACCESS_TOKEN_EXPIRATION_SECONDS_ENV_VAR_NAME, KEYPAIRS_STORE_PATH_ENV_VAR_NAME,
        POSTGRESDB_MAX_CONNECTIONS_ENV_VAR_NAME, POSTGRESQL_URL_ENV_VAR_NAME,
        SERVER_ADDR_ENV_VAR_NAME, SESSION_EXPIRATION_SECONDS_ENV_VAR_NAME,
    },
    errors::ErrorBoxed,
};
use tokio::sync::oneshot;
use tracing::subscriber;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config_from_env().map_err(|boxed| boxed.inner())?;

    configure_tracing(&config).map_err(|boxed| boxed.inner())?;

    let use_cases = build_use_cases(&config)
        .await
        .map_err(|boxed| boxed.inner())?;

    let (_, shutdown_signal_receiver) = oneshot::channel();

    WebApi::serve(&config, use_cases, shutdown_signal_receiver).await?;

    Ok(())
}

fn get_config_from_env() -> Result<AppConfig, ErrorBoxed> {
    dotenvy::dotenv()?;

    let mut config_builder = AppConfigBuilder::new(AppConfigRequiredOptions {
        server_addr: env::var(SERVER_ADDR_ENV_VAR_NAME)?,
        keypairs_store_path: env::var(KEYPAIRS_STORE_PATH_ENV_VAR_NAME)?.parse()?,
        postgres_db_url: env::var(POSTGRESQL_URL_ENV_VAR_NAME)?.parse()?,
    });

    if let Ok(value) = env::var(SESSION_EXPIRATION_SECONDS_ENV_VAR_NAME) {
        let parsed = value.parse()?;
        config_builder.with_session_expiration_seconds(parsed);
    };

    if let Ok(value) = env::var(ACCESS_TOKEN_EXPIRATION_SECONDS_ENV_VAR_NAME) {
        let parsed = value.parse()?;
        config_builder.with_access_token_expiration_seconds(parsed);
    }

    if let Ok(value) = env::var(POSTGRESDB_MAX_CONNECTIONS_ENV_VAR_NAME) {
        let parsed = value.parse()?;
        config_builder.with_postgres_db_max_connections(parsed);
    }

    Ok(config_builder.build())
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
    let keypair_repository =
        Arc::new(FileSystemKeyPairRepository::init(app_config.keypairs_store_path()).await?);
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
