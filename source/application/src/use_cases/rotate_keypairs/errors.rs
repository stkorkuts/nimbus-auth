use nimbus_auth_domain::entities::keypair::value_objects::errors::KeyPairValueError;
use nimbus_auth_shared::types::UserRole;
use thiserror::Error;

use crate::services::{
    keypair_repository::errors::KeyPairRepositoryError, random_service::errors::RandomServiceError,
    time_service::errors::TimeServiceError,
};

#[derive(Debug, Error)]
pub enum RotateKeyPairsError {
    #[error("operation forbiddden for a user with a role: {0}")]
    Forbidden(UserRole),
    #[error(transparent)]
    RandomService(#[from] RandomServiceError),
    #[error(transparent)]
    KeyPairValue(#[from] KeyPairValueError),
    #[error(transparent)]
    KeyPairRepository(#[from] KeyPairRepositoryError),
    #[error(transparent)]
    TimeService(#[from] TimeServiceError),
}
