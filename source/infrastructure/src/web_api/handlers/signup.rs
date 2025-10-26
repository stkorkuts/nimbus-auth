use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode},
    response::IntoResponse,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use nimbus_auth_application::use_cases::{
    SessionDto, SignUpError, SignUpRequest, SignUpResponse, UseCases,
};
use nimbus_auth_proto::proto::nimbus::auth::signup::v1::{
    SignUpErrorCodeProto, SignUpRequestProto, SignUpResponseProto, SignUpSuccessResponseProto,
    sign_up_response_proto::{self},
};
use nimbus_auth_shared::errors::ErrorBoxed;
use prost::Message;
use tracing::error;
use zeroize::Zeroizing;

use crate::{
    converters::{convert_access_token_into_proto, convert_user_into_proto},
    web_api::{
        extractors::client_extractor::{Client, ClientType},
        responses::proto::ProtoResponse,
    },
};

pub async fn handle_signup(
    State(use_cases): State<UseCases>,
    Client(client_type): Client,
    body: Bytes,
) -> impl IntoResponse {
    let SignUpRequestProto {
        user_name,
        password,
    } = match SignUpRequestProto::decode(body) {
        Ok(request) => request,
        Err(_) => {
            return ProtoResponse::new(
                StatusCode::BAD_REQUEST,
                SignUpResponseProto {
                    result: Some(sign_up_response_proto::Result::Error(
                        SignUpErrorCodeProto::WrongBodyFormat.into(),
                    )),
                },
            );
        }
    };
    let password = Zeroizing::new(password);

    let result = use_cases
        .signup(SignUpRequest {
            user_name: &user_name,
            password: &password,
        })
        .await;

    match result {
        Ok(response) => match ProtoResponse::new(
            StatusCode::CREATED,
            SignUpResponseProto {
                result: Some(sign_up_response_proto::Result::Success(
                    SignUpSuccessResponseProto {
                        user: Some(convert_user_into_proto(response.user)),
                        access_token: Some(convert_access_token_into_proto(response.access_token)),
                    },
                )),
            },
        )
        .with_session_headers(client_type, &response.session)
        {
            Ok(response_with_session_headers) => response_with_session_headers,
            Err(err) => {
                error!("internal error in handle_signup (handle_successful_signup): {err}");
                ProtoResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    SignUpResponseProto {
                        result: Some(sign_up_response_proto::Result::Error(
                            SignUpErrorCodeProto::Undefined.into(),
                        )),
                    },
                )
            }
        },
        Err(err) => match err {
            SignUpError::InvalidUserName(_) | SignUpError::InvalidPassword(_) => {
                ProtoResponse::new(
                    StatusCode::BAD_REQUEST,
                    SignUpResponseProto {
                        result: Some(sign_up_response_proto::Result::Error(
                            SignUpErrorCodeProto::ValidationError.into(),
                        )),
                    },
                )
            }
            SignUpError::UserAlreadyExists { .. } => ProtoResponse::new(
                StatusCode::BAD_REQUEST,
                SignUpResponseProto {
                    result: Some(sign_up_response_proto::Result::Error(
                        SignUpErrorCodeProto::UserAlreadyExists.into(),
                    )),
                },
            ),
            err => {
                error!("internal error in handle_signup: {err}");
                ProtoResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    SignUpResponseProto {
                        result: Some(sign_up_response_proto::Result::Error(
                            SignUpErrorCodeProto::Undefined.into(),
                        )),
                    },
                )
            }
        },
    }
}
