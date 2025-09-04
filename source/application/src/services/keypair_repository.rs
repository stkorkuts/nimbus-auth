use std::error::Error;

use nimbus_auth_domain::entities::keypair::{Active, InitializedKeyPair, KeyPair};
use nimbus_auth_shared::{errors::ErrorBoxed, futures::PinnedFuture};
use ulid::Ulid;

use crate::services::transactions::{Transaction, Transactional};

pub trait KeyPairRepository: Transactional<TransactionType = Transaction> + Send + Sync {
    fn get_by_id(
        &self,
        id: &Ulid,
        transaction: Option<Self::TransactionType>,
    ) -> PinnedFuture<Option<InitializedKeyPair>, ErrorBoxed>;
    fn get_active(
        &self,
        transaction: Option<Self::TransactionType>,
    ) -> PinnedFuture<Option<KeyPair<Active>>, ErrorBoxed>;
    fn save(
        &self,
        keypair: &InitializedKeyPair,
        transaction: Option<Self::TransactionType>,
    ) -> PinnedFuture<(), ErrorBoxed>;
}
