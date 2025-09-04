use nimbus_auth_shared::futures::PinnedFuture;
use time::OffsetDateTime;

pub trait TimeService: Send + Sync {
    fn get_current_time(&self) -> PinnedFuture<OffsetDateTime>;
}
