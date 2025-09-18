use std::{collections::HashMap, path::PathBuf, str::FromStr, sync::Arc};

use mockall::predicate;
use nimbus_auth_application::{
    services::user_repository::errors::UserRepositoryError,
    use_cases::{UseCases, UseCasesConfig, UseCasesServices},
};
use nimbus_auth_domain::{
    entities::{
        Entity,
        session::InitializedSession,
        user::{
            User,
            specifications::{NewUserSpecification, RestoreUserSpecification},
            value_objects::{name::UserName, password_hash::PasswordHash},
        },
    },
    value_objects::identifier::Identifier,
};
use nimbus_auth_infrastructure::{
    axum_api::WebApi,
    services_implementations::{
        os_random_service::OsRandomService, os_time_service::OsTimeService,
    },
};
use nimbus_auth_shared::{
    config::{AppConfig, AppConfigBuilder, AppConfigRequiredOptions},
    constants::{ACCESS_TOKEN_EXPIRATION_SECONDS_DEFAULT, SESSION_EXPIRATION_SECONDS_DEFAULT},
    errors::ErrorBoxed,
    futures::{pin_static_future, pin_static_future_error_boxed},
    types::{AccessTokenExpirationSeconds, SessionExpirationSeconds},
};
use tokio::{
    spawn,
    sync::{RwLock, oneshot},
};
use ulid::Ulid;

use crate::tests::{
    entities::user::TestUser,
    mocks::services::{
        keypair_repository::MockTestKeyPairRepository,
        session_repository::MockTestSessionRepository, user_repository::MockTestUserRepository,
    },
};

mod signup;

struct IntegrationTestState {
    pub users: Option<Vec<TestUser>>,
    pub sessions: Option<Vec<InitializedSession>>,
}

async fn run_integration_test<
    Fut: Future<Output = Result<(), ErrorBoxed>>,
    TAction: FnOnce() -> Fut,
>(
    action: TAction,
    config: AppConfig,
    state: IntegrationTestState,
) -> Result<(), ErrorBoxed> {
    let use_cases = build_use_cases(&state).await?;

    let (shutdown_signal_sender, shutdown_signal_receiver) = oneshot::channel();

    let join_handle = spawn(async move {
        WebApi::run(&config, use_cases, shutdown_signal_receiver).await?;
        Ok::<(), ErrorBoxed>(())
    });

    let test_result = action().await;

    shutdown_signal_sender
        .send(())
        .map_err(|_| ErrorBoxed::from_str("can not send webapi shutdown signal"))?;
    join_handle.await??;

    test_result
}

async fn build_use_cases(state: &IntegrationTestState) -> Result<UseCases, ErrorBoxed> {
    let use_cases_config = UseCasesConfig {
        session_expiration_seconds: SessionExpirationSeconds(SESSION_EXPIRATION_SECONDS_DEFAULT),
        access_token_expiration_seconds: AccessTokenExpirationSeconds(
            ACCESS_TOKEN_EXPIRATION_SECONDS_DEFAULT,
        ),
    };

    let user_repository = build_user_repository(state).await?;
    let session_repository = build_session_repository(state);
    let keypair_repository = build_keypair_repository(state);

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

async fn build_user_repository(
    state: &IntegrationTestState,
) -> Result<MockTestUserRepository, ErrorBoxed> {
    let mut repo = MockTestUserRepository::new();

    // Shared state with interior mutability
    let store: Arc<RwLock<HashMap<Identifier<Ulid, User>, TestUser>>> =
        Arc::new(RwLock::new(HashMap::new()));

    // Preload initial users if provided
    if let Some(users) = &state.users {
        let mut map = store.write().await;
        for u in users {
            map.insert(Identifier::from(u.id), u.clone());
        }
    }

    // --- get_by_id ---
    {
        let store = store.clone();
        repo.expect_get_by_id().returning(move |id| {
            let store = store.clone();
            pin_static_future(async move {
                store
                    .read()
                    .await
                    .get(&id)
                    .map(|u| u.clone().into_domain())
                    .transpose()
                    .map_err(UserRepositoryError::from)
            })
        });
    }

    // --- save ---
    {
        let store = store.clone();
        repo.expect_save().returning(move |user| {
            let store = store.clone();
            let user = TestUser::from(user);
            pin_static_future(async move {
                store.write().await.insert(Identifier::from(user.id), user);
                Ok(())
            })
        });
    }

    Ok(repo)
}

fn build_session_repository(state: &IntegrationTestState) -> MockTestSessionRepository {
    let repo = MockTestSessionRepository::new();

    repo
}

fn build_keypair_repository(state: &IntegrationTestState) -> MockTestKeyPairRepository {
    let repo = MockTestKeyPairRepository::new();

    repo
}
