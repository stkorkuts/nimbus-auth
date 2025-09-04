use std::sync::Arc;

use nimbus_auth_domain::entities::keypair::{
    InitializedKeyPair, KeyPair, Uninitialized, specifications::NewKeyPairSpecification,
};
use nimbus_auth_shared::config::AccessTokenExpirationSeconds;

use crate::{
    services::{
        keypair_repository::KeyPairRepository, time_service::TimeService,
        transactions::TransactionLike,
    },
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
        .map_err(RotateKeyPairsError::KeyPairsRepositoryError)?;

    let current_time = time_service
        .get_current_time()
        .await
        .map_err(RotateKeyPairsError::TimeServiceError)?;

    let result = (async {
        let active_keypair = keypair_repository
            .get_active(Some(transaction.clone()))
            .await
            .map_err(RotateKeyPairsError::KeyPairsRepositoryError)?;

        match active_keypair {
            Some(active_keypair) => {
                let new_pairs = active_keypair.rotate(current_time, expiration_seconds);
                keypair_repository
                    .save(
                        &InitializedKeyPair::from(new_pairs.0),
                        Some(transaction.clone()),
                    )
                    .await
                    .map_err(RotateKeyPairsError::KeyPairsRepositoryError)?;
                keypair_repository
                    .save(
                        &InitializedKeyPair::from(new_pairs.1),
                        Some(transaction.clone()),
                    )
                    .await
                    .map_err(RotateKeyPairsError::KeyPairsRepositoryError)?;
            }
            None => {
                let new_keypair = KeyPair::<Uninitialized>::new(NewKeyPairSpecification {});
                keypair_repository
                    .save(
                        &InitializedKeyPair::from(new_keypair),
                        Some(transaction.clone()),
                    )
                    .await
                    .map_err(RotateKeyPairsError::KeyPairsRepositoryError)?;
            }
        }

        Ok::<_, RotateKeyPairsError>(())
    })
    .await;

    return match result {
        Ok(()) => {
            transaction
                .commit()
                .await
                .map_err(RotateKeyPairsError::TransactionError)?;

            Ok(RotateKeyPairsResponse {})
        }
        Err(err) => {
            transaction
                .rollback()
                .await
                .map_err(RotateKeyPairsError::TransactionError)?;

            Err(err)
        }
    };
}
