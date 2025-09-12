use thiserror::Error;

#[derive(Debug, Error)]
pub enum PostgresDatabaseError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}
