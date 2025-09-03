use std::{env, error::Error};

use nimbus_auth_application::use_cases::UseCases;
use nimbus_auth_infrastructure::api::WebApi;
use nimbus_auth_shared::{
    config::{AppConfig, AppConfigBuilder, AppConfigRequiredOptions},
    constants::{
        ACCESS_TOKEN_EXPIRATION_SECONDS_ENV_VAR_NAME, SERVER_ADDR_ENV_VAR_NAME,
        SESSION_EXPIRATION_SECONDS_ENV_VAR_NAME,
    },
};
use tracing::subscriber;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config_from_env()?;

    configure_tracing(&config)?;

    let use_cases = build_use_cases(&config)?;

    WebApi::run(&config, use_cases).await?;

    Ok(())
}

fn get_config_from_env() -> Result<AppConfig, Box<dyn Error>> {
    dotenvy::dotenv()?;

    let mut config_builder = AppConfigBuilder::new(AppConfigRequiredOptions {
        server_addr: env::var(SERVER_ADDR_ENV_VAR_NAME)?,
    });

    if let Ok(value) = env::var(SESSION_EXPIRATION_SECONDS_ENV_VAR_NAME) {
        let parsed = value.parse()?;
        config_builder.with_session_expiration_seconds(parsed);
    };

    if let Ok(value) = env::var(ACCESS_TOKEN_EXPIRATION_SECONDS_ENV_VAR_NAME) {
        let parsed = value.parse()?;
        config_builder.with_access_token_expiration_seconds(parsed);
    }

    Ok(config_builder.build())
}

fn configure_tracing(_: &AppConfig) -> Result<(), Box<dyn Error>> {
    let subscriber = FmtSubscriber::builder().finish();

    subscriber::set_global_default(subscriber)?;

    Ok(())
}

fn build_use_cases(app_config: &AppConfig) -> Result<UseCases, Box<dyn Error>> {
    todo!();
}
