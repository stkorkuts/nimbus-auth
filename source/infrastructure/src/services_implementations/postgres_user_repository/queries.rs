use nimbus_auth_application::services::user_repository::errors::UserRepositoryError;
use nimbus_auth_shared::errors::ErrorBoxed;

use crate::services_implementations::postgres_user_repository::schema::{GetUserDb, SaveUserDb};

pub async fn get_user_by_id<'a, E>(
    executor: &'a mut E,
    id: &str,
) -> Result<Option<GetUserDb>, UserRepositoryError>
where
    &'a mut E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    Ok(
        sqlx::query_as::<_, GetUserDb>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(executor)
            .await
            .map_err(ErrorBoxed::from)?,
    )
}

pub async fn get_user_by_name<'a, E>(
    executor: &'a mut E,
    name: &str,
) -> Result<Option<GetUserDb>, UserRepositoryError>
where
    &'a mut E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    todo!()
}

pub async fn get_user_by_session<'a, E>(
    executor: &'a mut E,
    session_id: &str,
) -> Result<Option<GetUserDb>, UserRepositoryError>
where
    &'a mut E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    todo!()
}

pub async fn save_user<'a, E>(
    executor: &'a mut E,
    user: &SaveUserDb,
) -> Result<(), UserRepositoryError>
where
    &'a mut E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    todo!()
}
