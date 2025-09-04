use std::error::Error;

use nimbus_auth_domain::entities::session::InitializedSession;
use nimbus_auth_shared::{errors::ErrorBoxed, futures::PinnedFuture};
use ulid::Ulid;

use crate::services::transactions::{Transaction, Transactional};

pub trait SessionRepository: Transactional<TransactionType = Transaction> + Send + Sync {
    fn get_by_id(
        &self,
        id: &Ulid,
        transaction: Option<Self::TransactionType>,
    ) -> PinnedFuture<Option<InitializedSession>, ErrorBoxed>;
    fn save(
        &self,
        session: &InitializedSession,
        transaction: Option<Self::TransactionType>,
    ) -> PinnedFuture<(), ErrorBoxed>;
}
