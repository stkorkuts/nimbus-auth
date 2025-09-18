use std::{error::Error, path::PathBuf, str::FromStr, sync::Arc};

use nimbus_auth_application::use_cases::{UseCases, UseCasesConfig, UseCasesServices};
use nimbus_auth_infrastructure::axum_api::{WebApi, errors::WebApiError};
use nimbus_auth_shared::{
    config::{AppConfig, AppConfigBuilder, AppConfigRequiredOptions},
    constants::{ACCESS_TOKEN_EXPIRATION_SECONDS_DEFAULT, SESSION_EXPIRATION_SECONDS_DEFAULT},
    errors::ErrorBoxed,
    types::{AccessTokenExpirationSeconds, SessionExpirationSeconds},
};
use tokio::{spawn, sync::oneshot};

use crate::tests::integration::signup::success::mocks::MockTestUserRepository;

mod mocks;

const SERVER_ADDR: &str = "https://localhost:8080";
const KEYPAIRS_STORE_PATH: &str = "/temp";
const POSTGRES_DB_URL: &str =
    "postgresql://<username>:<password>@<host>:<port>/<database>?<options>";

#[tokio::test]
async fn test_signup_success() -> Result<(), Box<dyn Error>> {
    let use_cases: UseCases = build_use_cases();
    let (shutdown_signal_sender, shutdown_signal_receiver) = oneshot::channel();

    let join_handle = spawn(async {
        let app_config = AppConfigBuilder::new(AppConfigRequiredOptions {
            server_addr: SERVER_ADDR.to_string(),
            keypairs_store_path: PathBuf::from_str(KEYPAIRS_STORE_PATH)?,
            postgres_db_url: POSTGRES_DB_URL.to_string(),
        })
        .build();
        WebApi::run(&app_config, use_cases, shutdown_signal_receiver).await?;
        Ok::<(), ErrorBoxed>(())
    });

    let test_result = async {
        let test_result = run_test().await;

        shutdown_signal_sender
            .send(())
            .map_err(|_| ErrorBoxed::from_str("can not send webapi shutdown signal"))?;
        join_handle.await??;

        test_result
    }
    .await;

    test_result.map_err(|boxed| boxed.inner())
}

fn build_use_cases() -> UseCases {
    let use_cases_config = UseCasesConfig {
        session_expiration_seconds: SessionExpirationSeconds(SESSION_EXPIRATION_SECONDS_DEFAULT),
        access_token_expiration_seconds: AccessTokenExpirationSeconds(
            ACCESS_TOKEN_EXPIRATION_SECONDS_DEFAULT,
        ),
    };
    todo!();
    // let user_repository = MockTestUserRepository::new();

    // let use_cases_services = UseCasesServices {
    //     user_repository: Arc::new(user_repository)
    // }

    // UseCases::new(use_cases_config, use_cases_services)
}

async fn run_test() -> Result<(), ErrorBoxed> {
    Ok(())
}
