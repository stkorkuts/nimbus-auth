use nimbus_auth_application::services::time_service::{TimeService, errors::TimeServiceError};
use nimbus_auth_shared::futures::{PinnedFuture, pin};
use time::OffsetDateTime;

pub struct OsTimeService {}

impl TimeService for OsTimeService {
    fn get_current_time(&self) -> PinnedFuture<time::OffsetDateTime, TimeServiceError> {
        pin(async { Ok(OffsetDateTime::now_utc()) })
    }
}
