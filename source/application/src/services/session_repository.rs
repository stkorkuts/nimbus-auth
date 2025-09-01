use nimbus_auth_domain::entities::session::Session;
use nimbus_auth_shared::futures::PinnedFuture;

pub trait SessionRepository {
    fn get_by_value(&self, session_value: &str) -> PinnedFuture<Option<Session>>;
    fn save(&self, session: &Session) -> PinnedFuture<()>;
}
