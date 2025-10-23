use std::sync::Arc;

use nimbus_auth_application::use_cases::{UseCases, UseCasesConfig, UseCasesServices};
use nimbus_auth_domain::entities::{keypair::SomeKeyPair, session::SomeSession, user::User};
use nimbus_auth_infrastructure::{
    web_api::WebApi,
    services_implementations::{
        os_random_service::OsRandomService, os_time_service::OsTimeService,
    },
};
use nimbus_auth_shared::{config::AppConfig, errors::ErrorBoxed};
use nimbus_auth_tests::mocks::{
    datastore::MockDatastore,
    services::{
        keypair_repository::MockKeyPairRepository, session_repository::MockSessionRepository,
        user_repository::MockUserRepository,
    },
};
use tokio::{spawn, sync::oneshot};

mod signin;
mod signup;

struct ApiTestState<'a> {
    pub users: Option<Vec<User>>,
    pub sessions: Option<Vec<SomeSession<'a>>>,
    pub keypairs: Option<Vec<SomeKeyPair<'a>>>,
}

async fn run_api_test<Fut: Future<Output = Result<(), ErrorBoxed>>, TAction: FnOnce() -> Fut>(
    action: TAction,
    config: AppConfig,
    state: ApiTestState<'static>,
) -> Result<(), ErrorBoxed> {
    let use_cases = build_use_cases(&config, state).await?;

    let (shutdown_signal_sender, shutdown_signal_receiver) = oneshot::channel();

    let join_handle = spawn(async move {
        WebApi::serve(&config, use_cases, shutdown_signal_receiver).await?;
        Ok::<(), ErrorBoxed>(())
    });

    let test_result = action().await;

    shutdown_signal_sender
        .send(())
        .map_err(|_| ErrorBoxed::from_str("can not send webapi shutdown signal"))?;
    join_handle.await??;

    test_result
}

async fn build_use_cases(
    config: &AppConfig,
    state: ApiTestState<'static>,
) -> Result<UseCases, ErrorBoxed> {
    let use_cases_config = UseCasesConfig {
        session_expiration_seconds: config.session_expiration_seconds(),
        access_token_expiration_seconds: config.access_token_expiration_seconds(),
    };

    let datastore = Arc::new(MockDatastore::new(
        state.users,
        state.sessions,
        state.keypairs,
    ));

    let user_repository = MockUserRepository::new(datastore.clone());
    let session_repository = MockSessionRepository::new(datastore.clone());
    let keypair_repository = MockKeyPairRepository::new(datastore.clone());

    let time_service = OsTimeService::new();
    let random_service = OsRandomService::new();

    let use_cases_services = UseCasesServices {
        user_repository: Arc::new(user_repository),
        session_repository: Arc::new(session_repository),
        keypair_repository: Arc::new(keypair_repository),
        time_service: Arc::new(time_service),
        random_service: Arc::new(random_service),
    };

    Ok(UseCases::new(use_cases_config, use_cases_services))
}
