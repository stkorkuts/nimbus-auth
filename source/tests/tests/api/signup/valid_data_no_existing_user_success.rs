use std::{error::Error, path::PathBuf, str::FromStr, time::Duration};

use nimbus_auth_domain::entities::keypair::SomeKeyPair;
use nimbus_auth_proto::proto::nimbus::auth::signup::v1::{
    SignUpRequestProto, SignUpResponseProto, sign_up_response_proto,
};
use nimbus_auth_shared::{
    config::{AppConfigBuilder, AppConfigRequiredOptions},
    constants::{
        CLIENT_TYPE_HEADER_NAME, CLIENT_TYPE_PC_HEADER_VALUE, SESSION_EXPIRATION_SECONDS_DEFAULT,
        SESSION_HEADER_EXP_TIMESTAMP_NAME, SESSION_HEADER_NAME,
    },
    errors::ErrorBoxed,
};
use nimbus_auth_tests::utils::get_active_keypair;
use prost::Message;
use reqwest::{Client, header::CONTENT_TYPE};
use time::OffsetDateTime;
use tracing::debug;
use ulid::Ulid;

use crate::api::{ApiTestState, run_api_test};

const SERVER_ADDR: &str = "localhost:5001";
const KEYPAIRS_STORE_PATH: &str = "/temp";
const POSTGRES_DB_URL: &str =
    "postgresql://<username>:<password>@<host>:<port>/<database>?<options>";

const VALID_USER_NAME: &str = "stanislau";
const VALID_PASSWORD: &str = "StrongPassword123!";

const ENDPOINT: &str = "auth/signup";

#[tokio::test]
async fn valid_data_no_existing_user() -> Result<(), Box<dyn Error>> {
    let app_config = AppConfigBuilder::new(AppConfigRequiredOptions {
        server_addr: SERVER_ADDR.to_string(),
        keypairs_store_path: PathBuf::from_str(KEYPAIRS_STORE_PATH)?,
        postgres_db_url: POSTGRES_DB_URL.to_string(),
    })
    .build()?;

    let active_keypair = get_active_keypair();

    let test_state = ApiTestState {
        users: None,
        sessions: None,
        keypairs: Some(vec![SomeKeyPair::from(active_keypair)]),
    };

    run_api_test(test_action, app_config, test_state)
        .await
        .map_err(|boxed| boxed.inner())
}

async fn test_action() -> Result<(), ErrorBoxed> {
    // arrange
    let test_start = OffsetDateTime::now_utc();

    // act
    let signup_request_proto = SignUpRequestProto {
        user_name: VALID_USER_NAME.to_string(),
        password: VALID_PASSWORD.to_string(),
    };
    let mut request_payload = Vec::new();
    signup_request_proto.encode(&mut request_payload)?;

    let client = Client::new();
    let response = client
        .post(format!("http://{SERVER_ADDR}/{ENDPOINT}"))
        .header(CONTENT_TYPE, "application/x-protobuf")
        .header(CLIENT_TYPE_HEADER_NAME, CLIENT_TYPE_PC_HEADER_VALUE)
        .body(request_payload)
        .send()
        .await?;

    let session_header_value = response
        .headers()
        .get(SESSION_HEADER_NAME)
        .ok_or(ErrorBoxed::from_str("session header is not found"))?
        .to_str()
        .map_err(|_| ErrorBoxed::from_str("session header has invalid value"))?;

    Ulid::from_str(session_header_value)
        .map_err(|_| ErrorBoxed::from_str("session header value is not ulid"))?;

    let session_header_exp_timestamp = response
        .headers()
        .get(SESSION_HEADER_EXP_TIMESTAMP_NAME)
        .ok_or(ErrorBoxed::from_str(
            "session header exp timestamp is not found",
        ))?
        .to_str()
        .map_err(|_| ErrorBoxed::from_str("session header exp timestamp has invalid value"))?;

    let session_header_exp_timestamp =
        OffsetDateTime::from_unix_timestamp(session_header_exp_timestamp.parse().map_err(
            |_| ErrorBoxed::from_str("session header exp timestamp value is not a number"),
        )?)
        .map_err(|_| {
            ErrorBoxed::from_str("session header exp timestamp value is not a valid unix timestamp")
        })?;

    let exp_diff = session_header_exp_timestamp - test_start;
    let eps = Duration::from_secs(10u64);
    debug!("exp_diff: {:?}", exp_diff);

    if exp_diff < Duration::from_secs(SESSION_EXPIRATION_SECONDS_DEFAULT as u64) - eps {
        return Err(ErrorBoxed::from_str(
            "session expiration time is lower than expected",
        ));
    }

    if exp_diff > Duration::from_secs(SESSION_EXPIRATION_SECONDS_DEFAULT as u64) + eps {
        return Err(ErrorBoxed::from_str(
            "session expiration time is bigger than expected",
        ));
    }

    let response_payload = response.bytes().await?;

    let signup_response_proto = SignUpResponseProto::decode(response_payload)?;

    //assert
    let success_signup_response_proto = match signup_response_proto.result.unwrap() {
        sign_up_response_proto::Result::Success(result) => result,
        sign_up_response_proto::Result::Error(error_code) => {
            return Err(ErrorBoxed::from_str(format!(
                "got error code from api: {error_code}"
            )));
        }
    };

    success_signup_response_proto
        .access_token
        .ok_or(ErrorBoxed::from_str("got empty access token from api"))?;

    let user_proto = success_signup_response_proto
        .user
        .ok_or(ErrorBoxed::from_str("got empty user from api"))?;

    if user_proto.user_name != VALID_USER_NAME {
        return Err(ErrorBoxed::from_str(format!(
            "user_name changed when calling api: from {} to {}",
            VALID_USER_NAME, user_proto.user_name
        )));
    }

    Ok(())
}
