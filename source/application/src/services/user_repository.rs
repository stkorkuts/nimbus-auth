use nimbus_auth_domain::entities::{
    session::{Active, Session},
    user::User,
};
use nimbus_auth_shared::futures::PinnedFuture;
use ulid::Ulid;

pub trait UserRepository: Send + Sync {
    fn get_by_id(&self, id: &Ulid) -> PinnedFuture<Option<User>>;
    fn get_by_username(&self, username: &str) -> PinnedFuture<Option<User>>;
    fn get_by_session(&self, refresh_token: &Session<Active>) -> PinnedFuture<Option<User>>;
    fn save(&self, user: &User) -> PinnedFuture<()>;
}
