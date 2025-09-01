use time::UtcDateTime;

use crate::entities::session::specifications::RestoreSessionSpecification;

pub mod specifications;

pub enum Session {
    Active(ActiveSession),
    Revoked(RevokedSession),
    Expired(ExpiredSession),
}

pub struct ActiveSession {
    value: String,
}
pub struct RevokedSession {
    revoked_at: UtcDateTime,
}
pub struct ExpiredSession {
    expired_at: UtcDateTime,
}

impl Session {
    pub fn new() -> ActiveSession {
        ActiveSession {
            value: "Test".to_owned(),
        }
    }

    pub fn restore(specs: RestoreSessionSpecification) -> Self {
        match specs.revoked_at {
            Some(revoked_at) => Self::Revoked(RevokedSession {
                revoked_at: revoked_at,
            }),
            None => match (specs.expires_at - specs.current_time).whole_seconds() > 0 {
                true => Self::Active(ActiveSession { value: specs.value }),
                false => Self::Expired(ExpiredSession {
                    expired_at: specs.expires_at,
                }),
            },
        }
    }
}

impl<'a> ActiveSession {
    pub fn revoke(self, current_time: UtcDateTime) -> RevokedSession {
        RevokedSession {
            revoked_at: current_time,
        }
    }

    pub fn value(&'a self) -> &'a str {
        &self.value
    }
}

impl RevokedSession {
    pub fn revoked_at(&self) -> UtcDateTime {
        self.revoked_at
    }
}

impl ExpiredSession {
    pub fn expired_at(&self) -> UtcDateTime {
        self.expired_at
    }
}
