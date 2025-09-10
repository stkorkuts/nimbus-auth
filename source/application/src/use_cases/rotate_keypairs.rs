use std::sync::Arc;

use nimbus_auth_domain::entities::keypair::{
    InitializedKeyPair, KeyPair, Uninitialized, specifications::NewKeyPairSpecification,
};
use nimbus_auth_shared::config::AccessTokenExpirationSeconds;

use crate::{
    services::{
        keypair_repository::KeyPairRepository,
        time_service::TimeService,
        transactions::{TransactionIsolationLevel, TransactonBlockTarget},
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
        .start_transaction(
            TransactionIsolationLevel::Serializable,
            TransactonBlockTarget::Table,
        )
        .await?;

    transaction
        .run(async move |inner_transaction| {
            let current_time = time_service.get_current_time().await?;

            let active_keypair = keypair_repository
                .get_active(Some(inner_transaction.clone()))
                .await?;

            match active_keypair {
                Some(active_keypair) => {
                    let new_pairs = active_keypair.rotate(current_time, expiration_seconds);
                    keypair_repository
                        .save(
                            &InitializedKeyPair::from(new_pairs.0),
                            Some(inner_transaction.clone()),
                        )
                        .await?;
                    keypair_repository
                        .save(
                            &InitializedKeyPair::from(new_pairs.1),
                            Some(inner_transaction.clone()),
                        )
                        .await?;
                    Ok(())
                }
                None => {
                    let new_keypair = KeyPair::<Uninitialized>::new(NewKeyPairSpecification {});
                    keypair_repository
                        .save(
                            &InitializedKeyPair::from(new_keypair),
                            Some(inner_transaction.clone()),
                        )
                        .await?;
                    Ok(())
                }
            }
        })
        .await?;

    Ok(RotateKeyPairsResponse {})
}
