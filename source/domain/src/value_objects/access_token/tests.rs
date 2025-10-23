use argon2::password_hash::SaltString;
use ed25519_dalek::{SigningKey, pkcs8::EncodePrivateKey, pkcs8::spki::der::pem::LineEnding};
use nimbus_auth_shared::{
    constants::ACCESS_TOKEN_EXPIRATION_SECONDS_DEFAULT, types::AccessTokenExpirationSeconds,
};
use rand::rngs::OsRng;
use time::OffsetDateTime;
use zeroize::Zeroizing;

use crate::{
    entities::{
        Entity,
        keypair::{
            Active, KeyPair, SomeKeyPair, specifications::NewKeyPairSpecification,
            value_objects::KeyPairValue,
        },
        user::{
            User,
            specifications::NewUserSpecification,
            value_objects::{password::Password, password_hash::PasswordHash, user_name::UserName},
        },
    },
    value_objects::{
        access_token::{AccessToken, errors::VerifyError},
        identifier::Identifier,
    },
};

const VALID_USER_NAME: &str = "validuser123";
const VALID_PASSWORD: &str = "StrongPassword123!";

fn get_user() -> User {
    let user_name = UserName::from(VALID_USER_NAME)
        .expect("username should have been constructed successfully");
    let password = Password::from(&Zeroizing::new(VALID_PASSWORD.to_string()))
        .expect("password should have been constructed successfully");
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = PasswordHash::hash(password, salt.as_str())
        .expect("password should have been cached successfully");

    User::new(NewUserSpecification {
        user_name: user_name,
        password_hash: password_hash,
    })
}

fn get_keypair() -> KeyPair<Active> {
    let mut rng = OsRng;
    let signing_key = SigningKey::generate(&mut rng);
    let pem = signing_key.to_pkcs8_pem(LineEnding::LF).unwrap();
    SomeKeyPair::new(NewKeyPairSpecification {
        value: KeyPairValue::from_pem(pem).expect("key pair value should have been constructed"),
    })
}

#[test]
fn encode_decode() {
    let user = get_user();
    let keypair = get_keypair();

    let access_token = AccessToken::new(
        user.claims().clone(),
        OffsetDateTime::now_utc(),
        AccessTokenExpirationSeconds(ACCESS_TOKEN_EXPIRATION_SECONDS_DEFAULT),
    );
    let signed_token = access_token
        .sign(&keypair)
        .expect("token should have been signed successfully");

    let result = AccessToken::verify_with_active(&signed_token, &keypair);
    assert!(matches!(result, Ok(..)));
}

#[test]
fn verify_key_extraction() {
    let user = get_user();
    let keypair = get_keypair();

    let access_token = AccessToken::new(
        user.claims().clone(),
        OffsetDateTime::now_utc(),
        AccessTokenExpirationSeconds(ACCESS_TOKEN_EXPIRATION_SECONDS_DEFAULT),
    );
    let signed_token = access_token
        .sign(&keypair)
        .expect("token should have been signed successfully");

    let keypair_id: Identifier<ulid::Ulid, KeyPair<Active>> =
        AccessToken::extract_keypair_id(&signed_token)
            .expect("signed token should have contained valid keypair id")
            .as_other_entity();

    assert_eq!(keypair.id(), &keypair_id);
}

#[test]
fn encode_decode_with_wrong_key() {
    let user = get_user();
    let keypair = get_keypair();
    let wrong_keypair = get_keypair();

    let access_token = AccessToken::new(
        user.claims().clone(),
        OffsetDateTime::now_utc(),
        AccessTokenExpirationSeconds(ACCESS_TOKEN_EXPIRATION_SECONDS_DEFAULT),
    );
    let signed_token = access_token
        .sign(&keypair)
        .expect("token should have been signed successfully");

    let result = AccessToken::verify_with_active(&signed_token, &wrong_keypair);
    assert!(matches!(result, Err(VerifyError::KeyPairIdsDoNotMatch)));
}

#[test]
fn modified_token() {
    let user = get_user();
    let keypair = get_keypair();

    let access_token = AccessToken::new(
        user.claims().clone(),
        OffsetDateTime::now_utc(),
        AccessTokenExpirationSeconds(ACCESS_TOKEN_EXPIRATION_SECONDS_DEFAULT),
    );
    let signed_token = access_token
        .sign(&keypair)
        .expect("token should have been signed successfully");

    let mut token_parts: Vec<String> = signed_token
        .split(".")
        .map(|part| part.to_string())
        .collect();
    let mut payload_bytes = token_parts[1].as_bytes().to_vec();
    let middle = payload_bytes.len() / 2;
    if payload_bytes[middle] == b'a' {
        payload_bytes[middle] = b'b';
    } else {
        payload_bytes[middle] = b'a';
    }
    token_parts[1] =
        String::from_utf8(payload_bytes).expect("payload bytes should still be valid utf8 string");
    let tampered_token = token_parts.join(".");

    let result = AccessToken::verify_with_active(&tampered_token, &keypair);
    assert!(matches!(result, Err(VerifyError::Decoding(..))));
}
