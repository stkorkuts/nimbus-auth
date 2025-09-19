use std::sync::Arc;

use nimbus_auth_domain::entities::keypair::{
    KeyPair, SomeKeyPair, SomeKeyPairRef, specifications::NewKeyPairSpecification,
    value_objects::KeyPairValue,
};
use nimbus_auth_shared::types::AccessTokenExpirationSeconds;

use crate::{
    services::{
        keypair_repository::KeyPairRepository, random_service::RandomService,
        time_service::TimeService,
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
    let private_key_pem = random_service.get_random_private_key_pem().await?;
    let keypair_value = KeyPairValue::from(&private_key_pem)?;

    let transactional_keypair_repository = keypair_repository.start_transaction().await?;

    let (transactional_keypair_repository, active_keypair) =
        transactional_keypair_repository.get_active().await?;

    let transactional_keypair_repository = match active_keypair {
        Some(active_keypair) => {
            let (expiring_keypair, new_active_keypair) = active_keypair.rotate(
                keypair_value,
                time_service.get_current_time().await?,
                expiration_seconds,
            );
            let (transactional_keypair_repository, _) = transactional_keypair_repository
                .save(SomeKeyPairRef::from(&expiring_keypair))
                .await?;
            let (transactional_keypair_repository, _) = transactional_keypair_repository
                .save(SomeKeyPairRef::from(&new_active_keypair))
                .await?;
            transactional_keypair_repository
        }
        None => {
            let new_keypair = SomeKeyPair::new(NewKeyPairSpecification {
                value: keypair_value,
            });
            let (transactional_keypair_repository, _) = transactional_keypair_repository
                .save(SomeKeyPairRef::from(&new_keypair))
                .await?;
            transactional_keypair_repository
        }
    };

    transactional_keypair_repository.commit().await?;

    Ok(RotateKeyPairsResponse {})
}
