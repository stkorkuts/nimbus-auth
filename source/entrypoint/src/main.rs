use std::{env, error::Error};

use nimbus_auth_shared::{
    config::{AppConfig, AppConfigBuilder},
    constants::{
        ACCESS_TOKEN_EXPIRATION_SECONDS_ENV_VAR_NAME, SESSION_EXPIRATION_SECONDS_ENV_VAR_NAME,
    },
};
use tracing::subscriber;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config_from_env()?;

    configure_tracing(config)?;

    Ok(())
}

fn get_config_from_env() -> Result<AppConfig, Box<dyn Error>> {
    dotenvy::dotenv()?;
    let mut config_builder = AppConfigBuilder::new();

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

fn configure_tracing(_: AppConfig) -> Result<(), Box<dyn Error>> {
    let subscriber = FmtSubscriber::builder().finish();

    subscriber::set_global_default(subscriber)?;

    Ok(())
}
