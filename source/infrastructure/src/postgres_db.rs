use std::time::Duration;

use nimbus_auth_shared::config::AppConfig;
use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::postgres_db::errors::PostgresDatabaseError;

pub mod errors;

pub struct PostgresDatabase {
    pool: PgPool,
}

impl PostgresDatabase {
    pub async fn new(config: &AppConfig) -> Result<Self, PostgresDatabaseError> {
        let pool = PgPoolOptions::new()
            .max_connections(config.postgres_db_max_connections().0)
            .acquire_timeout(Duration::from_secs(30))
            .idle_timeout(Duration::from_secs(600))
            .max_lifetime(Duration::from_secs(1800))
            .connect(config.postgres_db_url())
            .await?;
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
