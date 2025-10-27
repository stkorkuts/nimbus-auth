use axum::{extract::State, http::StatusCode, response::IntoResponse};
use nimbus_auth_application::use_cases::{RefreshError, RefreshRequest, UseCases};
use nimbus_auth_proto::proto::nimbus::auth::refresh::v1::{
    RefreshErrorCodeProto, RefreshResponseProto, RefreshSuccessResponseProto,
    refresh_response_proto,
};
use tracing::error;

use crate::{
    converters::{convert_access_token_into_proto, convert_user_into_proto},
    web_api::{
        extractors::{client_extractor::Client, session_extractor::Session},
        responses::proto::ProtoResponse,
    },
};

pub async fn handle_refresh(
    State(use_cases): State<UseCases>,
    Client(client_type): Client,
    Session { session_id }: Session,
) -> impl IntoResponse {
    let result = use_cases
        .refresh(RefreshRequest {
            session_id: &session_id,
        })
        .await;

    match result {
        Ok(response) => match ProtoResponse::new(
            StatusCode::OK,
            RefreshResponseProto {
                result: Some(refresh_response_proto::Result::Success(
                    RefreshSuccessResponseProto {
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
                error!("internal error in handle_refresh: {err}");
                ProtoResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    RefreshResponseProto {
                        result: Some(refresh_response_proto::Result::Error(
                            RefreshErrorCodeProto::Undefined.into(),
                        )),
                    },
                )
            }
        },
        Err(err) => match err {
            RefreshError::IdDecode(_) => ProtoResponse::new(
                StatusCode::BAD_REQUEST,
                RefreshResponseProto {
                    result: Some(refresh_response_proto::Result::Error(
                        RefreshErrorCodeProto::ValidationError.into(),
                    )),
                },
            ),
            RefreshError::SessionIsNotFound => ProtoResponse::new(
                StatusCode::BAD_REQUEST,
                RefreshResponseProto {
                    result: Some(refresh_response_proto::Result::Error(
                        RefreshErrorCodeProto::SessionNotFound.into(),
                    )),
                },
            ),
            RefreshError::UserIsNotFound
            | RefreshError::SessionIsExpired
            | RefreshError::SessionIsRevoked => ProtoResponse::new(
                StatusCode::BAD_REQUEST,
                RefreshResponseProto {
                    result: Some(refresh_response_proto::Result::Error(
                        RefreshErrorCodeProto::SessionInvalid.into(),
                    )),
                },
            ),
            err => {
                error!("internal error in handle_refresh: {err}");
                ProtoResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    RefreshResponseProto {
                        result: Some(refresh_response_proto::Result::Error(
                            RefreshErrorCodeProto::Undefined.into(),
                        )),
                    },
                )
            }
        },
    }
}
