use nimbus_auth_domain::entities::session::InitializedSession;
use nimbus_auth_shared::futures::PinnedFuture;
use ulid::Ulid;

pub trait SessionRepository: Send + Sync {
    fn get_by_id(&self, id: &Ulid) -> PinnedFuture<Option<InitializedSession>>;
    fn save(&self, session: &InitializedSession) -> PinnedFuture<()>;
}
