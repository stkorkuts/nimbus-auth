use nimbus_auth_domain::entities::session::OneOfSession;
use nimbus_auth_shared::futures::PinnedFuture;

pub trait SessionRepository {
    fn get_by_value(&self, session_value: &str) -> PinnedFuture<Option<OneOfSession>>;
    fn save(&self, session: &OneOfSession) -> PinnedFuture<()>;
}
