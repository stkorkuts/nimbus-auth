pub mod errors;

use jsonwebtoken::{EncodingKey, Header, encode};
use nimbus_auth_shared::{
    config::AccessTokenExpirationSeconds,
    constants::{ACCESS_TOKEN_AUDIENCE, ACCESS_TOKEN_ISSUER},
};
use serde::Serialize;
use time::OffsetDateTime;
use ulid::Ulid;

use crate::{
    entities::user::User,
    value_objects::{access_token::errors::SignAccessTokenError, identifier::Identifier},
};

pub struct AccessToken {
    user_id: Identifier<Ulid, User>,
    expires_at: OffsetDateTime,
}

#[derive(Debug, Serialize)]
struct Claims {
    aud: String,
    exp: usize,
    iss: String,
    sub: String,
}

impl AccessToken {
    pub fn new(
        user_id: Identifier<Ulid, User>,
        current_time: OffsetDateTime,
        AccessTokenExpirationSeconds(expiration_seconds): AccessTokenExpirationSeconds,
    ) -> AccessToken {
        AccessToken {
            user_id,
            expires_at: current_time + time::Duration::seconds(expiration_seconds as i64),
        }
    }

    pub fn user_id(&self) -> &Identifier<Ulid, User> {
        &self.user_id
    }

    pub fn expires_at(&self) -> &OffsetDateTime {
        &self.expires_at
    }

    pub fn sign(&self, rsa_pem_private_key: &[u8]) -> Result<String, SignAccessTokenError> {
        let header = Header::new(jsonwebtoken::Algorithm::RS256);

        let expiration_timestamp = self.expires_at.unix_timestamp() as usize;
        let claims = Claims {
            aud: ACCESS_TOKEN_AUDIENCE.to_string(),
            exp: expiration_timestamp,
            iss: ACCESS_TOKEN_ISSUER.to_string(),
            sub: self.user_id.to_string(),
        };

        let key = EncodingKey::from_rsa_pem(rsa_pem_private_key)
            .map_err(SignAccessTokenError::InvalidPrivateKeyFormat)?;

        let token = encode(&header, &claims, &key).map_err(SignAccessTokenError::EncodingError)?;

        Ok(token)
    }
}
