pub mod specifications;

use std::error::Error;

use time::UtcDateTime;
use ulid::Ulid;

use crate::{
    entities::user::User,
    value_objects::{
        access_token::specifications::{
            NewAccessTokenSpecification, RestoreAccessTokenSpecification,
        },
        identifier::Identifier,
    },
};

pub trait AccessTokenState {}

pub struct Uninitialized {}

pub struct Active {
    user_id: Identifier<Ulid, User>,
    expires_at: UtcDateTime,
}

pub struct Expired {
    expired_at: UtcDateTime,
}

pub struct AccessToken<State: AccessTokenState> {
    state: State,
}

pub enum InitializedAccessToken {
    Active(AccessToken<Active>),
    Expired(AccessToken<Expired>),
}

impl AccessTokenState for Uninitialized {}
impl AccessTokenState for Active {}
impl AccessTokenState for Expired {}

impl AccessToken<Uninitialized> {
    pub fn new(
        NewAccessTokenSpecification {
            user_id,
            current_time,
            expiration_seconds,
        }: NewAccessTokenSpecification,
    ) -> AccessToken<Active> {
        AccessToken {
            state: Active {
                user_id: user_id,
                expires_at: current_time + time::Duration::seconds(expiration_seconds as i64),
            },
        }
    }

    pub fn restore(
        RestoreAccessTokenSpecification {
            signed,
            secret,
            current_time,
        }: RestoreAccessTokenSpecification,
    ) -> Result<InitializedAccessToken, Box<dyn Error>> {
        let parsed = Self::parse_signed(&signed, &secret)?;
        Ok(match (parsed.1 - current_time).whole_seconds() > 0 {
            true => InitializedAccessToken::from(AccessToken {
                state: Expired {
                    expired_at: parsed.1,
                },
            }),
            false => InitializedAccessToken::from(AccessToken {
                state: Active {
                    user_id: parsed.0,
                    expires_at: parsed.1,
                },
            }),
        })
    }

    fn parse_signed(
        signed: &str,
        secret: &str,
    ) -> Result<(Identifier<Ulid, User>, UtcDateTime), Box<dyn Error>> {
        todo!("Implement enum error type for access token restoration and parsing algorithm");
    }
}

impl AccessToken<Active> {
    pub fn user_id(&self) -> &Identifier<Ulid, User> {
        &self.state.user_id
    }

    pub fn expires_at(&self) -> &UtcDateTime {
        &self.state.expires_at
    }

    pub fn sign(&self, secret: &str) -> String {
        todo!()
    }
}

impl AccessToken<Expired> {
    pub fn expired_at(&self) -> &UtcDateTime {
        &self.state.expired_at
    }
}

impl From<AccessToken<Active>> for InitializedAccessToken {
    fn from(value: AccessToken<Active>) -> Self {
        InitializedAccessToken::Active(value)
    }
}

impl From<AccessToken<Expired>> for InitializedAccessToken {
    fn from(value: AccessToken<Expired>) -> Self {
        InitializedAccessToken::Expired(value)
    }
}
