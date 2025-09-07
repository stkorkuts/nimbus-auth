use std::error::Error;

use nimbus_auth_shared::{errors::ErrorBoxed, futures::PinnedFuture};
use time::OffsetDateTime;

use crate::services::time_service::errors::TimeServiceError;

pub mod errors;

pub trait TimeService: Send + Sync {
    fn get_current_time(&self) -> PinnedFuture<OffsetDateTime, TimeServiceError>;
}
