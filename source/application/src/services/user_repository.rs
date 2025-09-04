use nimbus_auth_domain::entities::{
    session::{Active, Session},
    user::User,
};
use nimbus_auth_shared::futures::PinnedFuture;
use ulid::Ulid;

use crate::services::transactions::{TransactionWrapper, Transactional};

pub trait UserRepository:
    Transactional<TransactionType = TransactionWrapper> + Send + Sync
{
    fn get_by_id(
        &self,
        id: &Ulid,
        transaction: Option<Self::TransactionType>,
    ) -> PinnedFuture<Option<User>>;
    fn get_by_username(
        &self,
        username: &str,
        transaction: Option<Self::TransactionType>,
    ) -> PinnedFuture<Option<User>>;
    fn get_by_session(
        &self,
        refresh_token: &Session<Active>,
        transaction: Option<Self::TransactionType>,
    ) -> PinnedFuture<Option<User>>;
    fn save(&self, user: &User, transaction: Option<Self::TransactionType>) -> PinnedFuture<()>;
}
