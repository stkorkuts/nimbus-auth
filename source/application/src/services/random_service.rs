use nimbus_auth_shared::futures::PinnedFuture;

use crate::services::random_service::errors::RandomServiceError;

pub mod errors;

pub trait RandomService: Send + Sync {
    fn get_random_private_key_pem(&self) -> PinnedFuture<String, RandomServiceError>;
}
