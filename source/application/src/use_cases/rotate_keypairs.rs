use std::sync::Arc;

use nimbus_auth_domain::entities::keypair::{
    InitializedKeyPair, KeyPair, Uninitialized, specifications::NewKeyPairSpecification,
    value_objects::KeyPairValue,
};
use nimbus_auth_shared::types::AccessTokenExpirationSeconds;

use crate::{
    services::{
        keypair_repository::KeyPairRepository,
        random_service::RandomService,
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
    random_service: Arc<dyn RandomService>,
    expiration_seconds: AccessTokenExpirationSeconds,
) -> Result<RotateKeyPairsResponse, RotateKeyPairsError> {
    let mut transaction = keypair_repository
        .start_transaction(
            TransactionIsolationLevel::Serializable,
            TransactonBlockTarget::Table,
        )
        .await?;

    let private_key_pem = random_service.get_random_private_key_pem().await?;
    let keypair_value = KeyPairValue::from(&private_key_pem)?;

    transaction
        .run(async move |inner_transaction| {
            let active_keypair = keypair_repository
                .get_active(Some(inner_transaction.clone()))
                .await?;

            match active_keypair {
                Some(active_keypair) => {
                    let new_pairs = active_keypair.rotate(
                        keypair_value,
                        time_service.get_current_time().await?,
                        expiration_seconds,
                    );
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
                    let new_keypair = KeyPair::<Uninitialized>::new(NewKeyPairSpecification {
                        value: keypair_value,
                    });
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
