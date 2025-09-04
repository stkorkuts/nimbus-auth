use std::sync::Arc;

use nimbus_auth_domain::entities::session::InitializedSession;
use nimbus_auth_shared::futures::PinnedFuture;
use tokio::sync::Mutex;
use ulid::Ulid;

use crate::services::transactions::{TransactionWrapper, Transactional};

pub trait SessionRepository:
    Transactional<TransactionType = TransactionWrapper> + Send + Sync
{
    fn get_by_id(
        &self,
        id: &Ulid,
        transaction: Option<Self::TransactionType>,
    ) -> PinnedFuture<Option<InitializedSession>>;
    fn save(
        &self,
        session: &InitializedSession,
        transaction: Option<Self::TransactionType>,
    ) -> PinnedFuture<()>;
}
