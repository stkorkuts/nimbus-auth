use nimbus_auth_application::services::session_repository::errors::SessionRepositoryError;
use nimbus_auth_shared::errors::ErrorBoxed;

use crate::services_implementations::postgres_session_repository::schema::GetSessionDb;

pub async fn get_session_by_id<'a, E>(
    executor: &'a mut E,
    id: &str,
) -> Result<Option<GetSessionDb>, SessionRepositoryError>
where
    &'a mut E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    Ok(
        sqlx::query_as::<_, GetSessionDb>("SELECT * FROM sessions WHERE id = $1")
            .bind(id)
            .fetch_optional(executor)
            .await
            .map_err(ErrorBoxed::from)?,
    )
}
