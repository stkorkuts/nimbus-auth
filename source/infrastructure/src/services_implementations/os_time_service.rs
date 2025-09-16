use nimbus_auth_application::services::time_service::{TimeService, errors::TimeServiceError};
use nimbus_auth_shared::futures::{StaticPinnedFuture, pin_future};
use time::OffsetDateTime;

pub struct OsTimeService {}

impl TimeService for OsTimeService {
    fn get_current_time(&self) -> StaticPinnedFuture<time::OffsetDateTime, TimeServiceError> {
        pin_future(async { Ok(OffsetDateTime::now_utc()) })
    }
}
