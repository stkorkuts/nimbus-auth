use nimbus_auth_shared::futures::PinnedFuture;
use time::OffsetDateTime;

use crate::services::time_service::errors::TimeServiceError;

pub mod errors;

pub trait TimeService: Send + Sync {
    fn get_current_time(&self) -> PinnedFuture<OffsetDateTime, TimeServiceError>;
}
