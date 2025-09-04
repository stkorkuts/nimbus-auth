use nimbus_auth_domain::entities::keypair::{Active, InitializedKeyPair, KeyPair};
use nimbus_auth_shared::futures::PinnedFuture;
use ulid::Ulid;

use crate::services::transactions::{TransactionWrapper, Transactional};

pub trait KeyPairRepository:
    Transactional<TransactionType = TransactionWrapper> + Send + Sync
{
    fn get_by_id(
        &self,
        id: &Ulid,
        transaction: Option<Self::TransactionType>,
    ) -> PinnedFuture<Option<InitializedKeyPair>>;
    fn get_active(
        &self,
        transaction: Option<Self::TransactionType>,
    ) -> PinnedFuture<Option<KeyPair<Active>>>;
    fn save(
        &self,
        keypair: &InitializedKeyPair,
        transaction: Option<Self::TransactionType>,
    ) -> PinnedFuture<()>;
}
