use std::collections::HashSet;

use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, decode_header, encode,
};
use nimbus_auth_shared::{
    constants::{ACCESS_TOKEN_AUDIENCE, ACCESS_TOKEN_ISSUER},
    types::{AccessTokenExpirationSeconds, UserRole},
};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use ulid::Ulid;
use zeroize::Zeroizing;

use crate::{
    entities::{
        Entity,
        keypair::{Active, Expiring, KeyPair, SomeKeyPair},
        user::value_objects::user_name::UserName,
    },
    value_objects::{
        access_token::errors::{ExtractKeyIdError, SignAccessTokenError, VerifyError},
        identifier::{Identifier, IdentifierOfType},
        user_claims::UserClaims,
    },
};

pub mod errors;
#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct AccessToken {
    user_claims: UserClaims,
    expires_at: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,
    exp: usize,
    iss: String,
    sub: String,
    name: String,
    role: String,
}

impl AccessToken {
    pub fn new(
        user_claims: UserClaims,
        current_time: OffsetDateTime,
        AccessTokenExpirationSeconds(expiration_seconds): AccessTokenExpirationSeconds,
    ) -> AccessToken {
        AccessToken {
            user_claims,
            expires_at: current_time + time::Duration::seconds(expiration_seconds as i64),
        }
    }

    pub fn user_claims(&self) -> &UserClaims {
        &self.user_claims
    }

    pub fn expires_at(&self) -> &OffsetDateTime {
        &self.expires_at
    }

    pub fn sign(
        &self,
        keypair: &KeyPair<Active>,
    ) -> Result<Zeroizing<String>, SignAccessTokenError> {
        let mut header = Header::new(jsonwebtoken::Algorithm::EdDSA);
        header.kid = Some(keypair.id().to_string());

        let expiration_timestamp = self.expires_at.unix_timestamp() as usize;
        let claims = Claims {
            aud: ACCESS_TOKEN_AUDIENCE.to_string(),
            exp: expiration_timestamp,
            iss: ACCESS_TOKEN_ISSUER.to_string(),
            sub: self.user_claims.id().to_string(),
            name: self.user_claims.name().to_string(),
            role: self.user_claims.role().to_string(),
        };

        let key = EncodingKey::from_ed_pem(keypair.value().private_key_pem().as_bytes())
            .map_err(SignAccessTokenError::InvalidPrivateKeyFormat)?;

        let token = encode(&header, &claims, &key).map_err(SignAccessTokenError::Encoding)?;

        Ok(Zeroizing::new(token))
    }

    pub fn extract_keypair_id(
        signed_token: &str,
    ) -> Result<Identifier<Ulid, SomeKeyPair>, ExtractKeyIdError> {
        let header =
            decode_header(signed_token).map_err(|err| ExtractKeyIdError::HeaderDecoding(err))?;
        let key_id = header.kid.ok_or(ExtractKeyIdError::KeyIdIsMissing)?;
        Ok(Identifier::from(
            Ulid::from_string(&key_id).map_err(|err| ExtractKeyIdError::WrongKeyIdFormat(err))?,
        ))
    }

    pub fn verify_with_active(
        signed_token: &str,
        keypair: &KeyPair<Active>,
    ) -> Result<AccessToken, VerifyError> {
        AccessToken::verify(
            signed_token,
            keypair.id().clone().as_other_entity(),
            &keypair.value().public_key_pem(),
        )
    }

    pub fn verify_with_expiring(
        signed_token: &str,
        keypair: &KeyPair<Expiring>,
    ) -> Result<AccessToken, VerifyError> {
        AccessToken::verify(
            signed_token,
            keypair.id().clone().as_other_entity(),
            &keypair.value().public_key_pem(),
        )
    }

    fn verify(
        signed_token: &str,
        expected_keypair_id: Identifier<Ulid, SomeKeyPair>,
        public_key_pem: &str,
    ) -> Result<AccessToken, VerifyError> {
        let actual_key_id = Self::extract_keypair_id(signed_token)?;
        if actual_key_id != expected_keypair_id {
            return Err(VerifyError::KeyPairIdsDoNotMatch);
        }

        let mut validation = Validation::new(Algorithm::EdDSA);
        validation.set_audience(&[ACCESS_TOKEN_AUDIENCE]);
        let mut issuer = HashSet::with_capacity(1);
        issuer.insert(ACCESS_TOKEN_ISSUER.to_string());
        validation.iss = Some(issuer);

        let decoding_key = DecodingKey::from_ed_pem(public_key_pem.as_bytes())
            .map_err(|err| VerifyError::InvalidDecodingKey(err))?;

        let claims = decode::<Claims>(signed_token, &decoding_key, &validation)
            .map_err(|err| VerifyError::Decoding(err))?
            .claims;

        let user_id = Identifier::from(
            Ulid::from_string(claims.sub.as_str())
                .map_err(|err| VerifyError::InvalidClaims(err.to_string()))?,
        );
        let user_name = UserName::from(claims.name.as_str())
            .map_err(|err| VerifyError::InvalidClaims(err.to_string()))?;
        let user_role = UserRole::try_from(claims.role.as_str())
            .map_err(|err| VerifyError::InvalidClaims(err))?;
        let expires_at = OffsetDateTime::from_unix_timestamp(claims.exp as i64)
            .map_err(|err| VerifyError::InvalidClaims(err.to_string()))?;

        Ok(AccessToken {
            user_claims: UserClaims::new(user_id, user_name, user_role),
            expires_at,
        })
    }
}
