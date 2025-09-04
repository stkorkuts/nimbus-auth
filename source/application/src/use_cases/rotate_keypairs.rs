use std::sync::Arc;

use nimbus_auth_domain::entities::keypair::{
    InitializedKeyPair, KeyPair, Uninitialized, specifications::NewKeyPairSpecification,
};
use nimbus_auth_shared::{
    config::AccessTokenExpirationSeconds,
    errors::ErrorBoxed,
    futures::{PinnedFutureExt, pin, pin_error_boxed},
};

use crate::{
    services::{keypair_repository::KeyPairRepository, time_service::TimeService},
    use_cases::{RotateKeyPairsError, RotateKeyPairsRequest, RotateKeyPairsResponse},
};

pub mod errors;
pub mod schema;

pub async fn handle_rotate_keypairs(
    RotateKeyPairsRequest {}: RotateKeyPairsRequest,
    keypair_repository: Arc<dyn KeyPairRepository>,
    time_service: Arc<dyn TimeService>,
    expiration_seconds: AccessTokenExpirationSeconds,
) -> Result<RotateKeyPairsResponse, RotateKeyPairsError> {
    let mut transaction = keypair_repository
        .start_transaction()
        .await
        .map_err(|err| RotateKeyPairsError::TransactionError(err))?;

    transaction
        .run(async move |inner_transaction| {
            let current_time = time_service
                .get_current_time()
                .await
                .map_err(|err| RotateKeyPairsError::TimeServiceError(err))?;

            let active_keypair = keypair_repository
                .get_active(Some(inner_transaction.clone()))
                .await
                .map_err(|err| RotateKeyPairsError::KeyPairsRepositoryError(err))?;

            match active_keypair {
                Some(active_keypair) => {
                    let new_pairs = active_keypair.rotate(current_time, expiration_seconds);
                    keypair_repository
                        .save(
                            &InitializedKeyPair::from(new_pairs.0),
                            Some(inner_transaction.clone()),
                        )
                        .await
                        .map_err(|err| RotateKeyPairsError::KeyPairsRepositoryError(err))?;
                    keypair_repository
                        .save(
                            &InitializedKeyPair::from(new_pairs.1),
                            Some(inner_transaction.clone()),
                        )
                        .await
                        .map_err(|err| RotateKeyPairsError::KeyPairsRepositoryError(err))?;
                    Ok::<(), ErrorBoxed>(())
                }
                None => {
                    let new_keypair = KeyPair::<Uninitialized>::new(NewKeyPairSpecification {});
                    keypair_repository
                        .save(
                            &InitializedKeyPair::from(new_keypair),
                            Some(inner_transaction.clone()),
                        )
                        .await
                        .map_err(|err| RotateKeyPairsError::KeyPairsRepositoryError(err))?;
                    Ok::<(), ErrorBoxed>(())
                }
            }
        })
        .await
        .map_err(|err| RotateKeyPairsError::TransactionError(err))?;

    Ok(RotateKeyPairsResponse {})
}
