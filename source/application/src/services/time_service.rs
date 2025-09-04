use std::error::Error;

use nimbus_auth_shared::{errors::ErrorBoxed, futures::PinnedFuture};
use time::OffsetDateTime;

pub trait TimeService: Send + Sync {
    fn get_current_time(&self) -> PinnedFuture<OffsetDateTime, ErrorBoxed>;
}
