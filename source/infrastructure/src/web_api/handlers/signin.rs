use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use nimbus_auth_application::use_cases::{SignInError, SignInRequest, UseCases};
use nimbus_auth_proto::proto::nimbus::auth::signin::v1::{
    SignInErrorCodeProto, SignInRequestProto, SignInResponseProto, SignInSuccessResponseProto,
    sign_in_response_proto,
};
use prost::Message;
use tracing::error;
use zeroize::Zeroizing;

use crate::{
    converters::{convert_access_token_into_proto, convert_user_into_proto},
    web_api::{extractors::client_extractor::Client, responses::proto::ProtoResponse},
};

pub async fn handle_signin(
    State(use_cases): State<UseCases>,
    Client(client_type): Client,
    body: Bytes,
) -> impl IntoResponse {
    let SignInRequestProto {
        user_name,
        password,
    } = match SignInRequestProto::decode(body) {
        Ok(request) => request,
        Err(_) => {
            return ProtoResponse::new(
                StatusCode::BAD_REQUEST,
                SignInResponseProto {
                    result: Some(sign_in_response_proto::Result::Error(
                        SignInErrorCodeProto::WrongBodyFormat.into(),
                    )),
                },
            );
        }
    };
    let password = Zeroizing::new(password);

    let result = use_cases
        .signin(SignInRequest {
            user_name: &user_name,
            password: &password,
        })
        .await;

    match result {
        Ok(response) => match ProtoResponse::new(
            StatusCode::OK,
            SignInResponseProto {
                result: Some(sign_in_response_proto::Result::Success(
                    SignInSuccessResponseProto {
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
                error!("internal error in handle_signin: {err}");
                ProtoResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    SignInResponseProto {
                        result: Some(sign_in_response_proto::Result::Error(
                            SignInErrorCodeProto::Undefined.into(),
                        )),
                    },
                )
            }
        },
        Err(err) => match err {
            SignInError::InvalidUserName(_) | SignInError::InvalidPassword(_) => {
                ProtoResponse::new(
                    StatusCode::BAD_REQUEST,
                    SignInResponseProto {
                        result: Some(sign_in_response_proto::Result::Error(
                            SignInErrorCodeProto::ValidationError.into(),
                        )),
                    },
                )
            }
            SignInError::UserIsNotFound { .. } | SignInError::PasswordDoesNotMatchWithHash => {
                ProtoResponse::new(
                    StatusCode::BAD_REQUEST,
                    SignInResponseProto {
                        result: Some(sign_in_response_proto::Result::Error(
                            SignInErrorCodeProto::WrongCredentials.into(),
                        )),
                    },
                )
            }
            err => {
                error!("internal error in handle_signin: {err}");
                ProtoResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    SignInResponseProto {
                        result: Some(sign_in_response_proto::Result::Error(
                            SignInErrorCodeProto::Undefined.into(),
                        )),
                    },
                )
            }
        },
    }
}
