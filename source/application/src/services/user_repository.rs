use nimbus_auth_domain::entities::{session::ActiveSession, user::User};
use nimbus_auth_shared::futures::PinnedFuture;

pub trait UserRepository {
    fn get_by_id(&self, id: &str) -> PinnedFuture<Option<User>>;
    fn get_by_username(&self, username: &str) -> PinnedFuture<Option<User>>;
    fn get_by_session(&self, refresh_token: &ActiveSession) -> PinnedFuture<Option<User>>;
    fn save(&self, user: &User) -> PinnedFuture<()>;
}
