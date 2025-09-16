use nimbus_auth_domain::{
    entities::keypair::{Active, InitializedKeyPair, KeyPair, Uninitialized},
    value_objects::identifier::Identifier,
};
use nimbus_auth_shared::futures::StaticPinnedFuture;
use ulid::Ulid;

use crate::services::keypair_repository::errors::KeyPairRepositoryError;

pub mod errors;

pub trait KeyPairRepository: Send + Sync {
    fn get_by_id(
        &self,
        id: &Identifier<Ulid, KeyPair<Uninitialized>>,
    ) -> StaticPinnedFuture<Option<InitializedKeyPair>, KeyPairRepositoryError>;
    fn get_active(&self) -> StaticPinnedFuture<Option<KeyPair<Active>>, KeyPairRepositoryError>;
    fn save(&self, keypair: &InitializedKeyPair) -> StaticPinnedFuture<(), KeyPairRepositoryError>;
}
