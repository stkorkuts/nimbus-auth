use nimbus_auth_application::services::user_repository::errors::UserRepositoryError;
use nimbus_auth_shared::errors::ErrorBoxed;

use crate::services_implementations::postgres_user_repository::schema::UserDb;

pub async fn get_user_by_id<'a, E>(
    executor: &'a mut E,
    id: &str,
) -> Result<Option<UserDb>, UserRepositoryError>
where
    &'a mut E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    Ok(
        sqlx::query_as::<_, UserDb>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(executor)
            .await
            .map_err(ErrorBoxed::from)?,
    )
}

pub async fn get_user_by_name<'a, E>(
    executor: &'a mut E,
    name: &str,
) -> Result<Option<UserDb>, UserRepositoryError>
where
    &'a mut E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    todo!()
}

pub async fn get_user_by_session<'a, E>(
    executor: &'a mut E,
    session_id: &str,
) -> Result<Option<UserDb>, UserRepositoryError>
where
    &'a mut E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    todo!()
}

pub async fn save_user<'a, E>(executor: &'a mut E, user: &UserDb) -> Result<(), UserRepositoryError>
where
    &'a mut E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    todo!()
}
