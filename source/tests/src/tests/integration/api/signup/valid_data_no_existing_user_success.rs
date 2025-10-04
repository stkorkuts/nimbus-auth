use std::{error::Error, path::PathBuf, str::FromStr};

use nimbus_auth_domain::{
    entities::{
        Entity,
        keypair::{
            SomeKeyPair, specifications::NewKeyPairSpecification, value_objects::KeyPairValue,
        },
    },
    value_objects::identifier::{Identifier, IdentifierOfType},
};
use nimbus_auth_proto::proto::nimbus::auth::signup::v1::{
    SignUpRequestProto, SignUpResponseProto, sign_up_response_proto,
};
use nimbus_auth_shared::{
    config::{AppConfigBuilder, AppConfigRequiredOptions},
    errors::ErrorBoxed,
};
use prost::Message;
use reqwest::Client;
use zeroize::Zeroizing;

use crate::tests::integration::api::{ApiTestState, run_api_test};

const SERVER_ADDR: &str = "localhost:5001";
const KEYPAIRS_STORE_PATH: &str = "/temp";
const POSTGRES_DB_URL: &str =
    "postgresql://<username>:<password>@<host>:<port>/<database>?<options>";
const PRIVATE_KEY_PEM: &str = "-----BEGIN PRIVATE KEY-----
MC4CAQAwBQYDK2VwBCIEIMUBs5zfkuEGgSLwrUo2vln82Z8hUySsoI+dyA3AonDV
-----END PRIVATE KEY-----
";

#[tokio::test]
async fn valid_data_no_existing_user() -> Result<(), Box<dyn Error>> {
    let app_config = AppConfigBuilder::new(AppConfigRequiredOptions {
        server_addr: SERVER_ADDR.to_string(),
        keypairs_store_path: PathBuf::from_str(KEYPAIRS_STORE_PATH)?,
        postgres_db_url: POSTGRES_DB_URL.to_string(),
    })
    .build();

    let active_keypair = SomeKeyPair::new(NewKeyPairSpecification {
        value: KeyPairValue::from_pem(Zeroizing::new(PRIVATE_KEY_PEM.to_string()))?,
    });

    let test_state = ApiTestState {
        users: None,
        sessions: None,
        keypairs: Some(vec![SomeKeyPair::Active {
            id: active_keypair.id().clone().as_other_entity(),
            keypair: active_keypair,
        }]),
    };

    run_api_test(test_action, app_config, test_state)
        .await
        .map_err(|boxed| boxed.inner())
}

async fn test_action() -> Result<(), ErrorBoxed> {
    // arrange
    let user_name = "stanislau";
    let password = "StrongPassword123!";

    // act
    let signup_request_proto = SignUpRequestProto {
        user_name: user_name.to_string(),
        password: password.to_string(),
    };
    let mut request_payload = Vec::new();
    signup_request_proto.encode(&mut request_payload)?;

    let client = Client::new();
    let response = client
        .post(format!("{SERVER_ADDR}/signup"))
        .header("Content-Type", "application/x-protobuf")
        .body(request_payload)
        .send()
        .await?;
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

    if user_proto.user_name != user_name {
        return Err(ErrorBoxed::from_str(format!(
            "user_name changed when calling api: from {} to {}",
            user_name, user_proto.user_name
        )));
    }

    Ok(())
}
