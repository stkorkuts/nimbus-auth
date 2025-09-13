use nimbus_auth_domain::{
    entities::keypair::{Active, InitializedKeyPair, KeyPair, Uninitialized},
    value_objects::identifier::Identifier,
};
use nimbus_auth_shared::futures::PinnedFuture;
use ulid::Ulid;

use crate::services::keypair_repository::errors::KeyPairRepositoryError;

pub mod errors;

pub trait KeyPairRepository: Send + Sync {
    fn get_by_id(
        &self,
        id: &Identifier<Ulid, KeyPair<Uninitialized>>,
    ) -> PinnedFuture<Option<InitializedKeyPair>, KeyPairRepositoryError>;
    fn get_active(&self) -> PinnedFuture<Option<KeyPair<Active>>, KeyPairRepositoryError>;
    fn save(&self, keypair: &InitializedKeyPair) -> PinnedFuture<(), KeyPairRepositoryError>;
}
