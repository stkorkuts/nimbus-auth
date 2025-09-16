use nimbus_auth_domain::entities::keypair::value_objects::errors::KeyPairValueError;
use thiserror::Error;

use crate::services::random_service::errors::RandomServiceError;

#[derive(Debug, Error)]
pub enum RotateKeyPairsError {
    #[error(transparent)]
    RandomService(#[from] RandomServiceError),
    #[error(transparent)]
    KeyPairValue(#[from] KeyPairValueError),
}
