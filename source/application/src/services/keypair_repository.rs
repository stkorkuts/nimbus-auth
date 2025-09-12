use nimbus_auth_domain::{
    entities::keypair::{Active, InitializedKeyPair, KeyPair, Uninitialized},
    value_objects::identifier::Identifier,
};
use nimbus_auth_shared::futures::PinnedFuture;
use ulid::Ulid;

use crate::services::{
    keypair_repository::errors::KeyPairRepositoryError,
    transactions::{Transaction, Transactional},
};

pub mod errors;

pub trait KeyPairRepository: Transactional<TransactionType = Transaction> + Send + Sync {
    fn get_by_id(
        &self,
        id: &Identifier<Ulid, KeyPair<Uninitialized>>,
        transaction: Option<Self::TransactionType>,
    ) -> PinnedFuture<Option<InitializedKeyPair>, KeyPairRepositoryError>;
    fn get_active(
        &self,
        transaction: Option<Self::TransactionType>,
    ) -> PinnedFuture<Option<KeyPair<Active>>, KeyPairRepositoryError>;
    fn save(
        &self,
        keypair: &InitializedKeyPair,
        transaction: Option<Self::TransactionType>,
    ) -> PinnedFuture<(), KeyPairRepositoryError>;
}
