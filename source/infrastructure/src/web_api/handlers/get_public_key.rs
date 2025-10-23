use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use nimbus_auth_application::use_cases::{GetPublicKeyError, GetPublicKeyRequest, UseCases};
use nimbus_auth_proto::proto::nimbus::auth::get_public_key::v1::{
    GetPublicKeyErrorCodeProto, GetPublicKeyResponseProto, GetPublicKeySuccessResponseProto,
    get_public_key_response_proto::Result,
};
use tracing::error;

use crate::web_api::responses::proto::ProtoResponse;

pub async fn handle_get_active_public_key(State(use_cases): State<UseCases>) -> impl IntoResponse {
    handle_get_public_key(use_cases, None).await
}

pub async fn handle_get_public_key_by_id(
    State(use_cases): State<UseCases>,
    Path(key_id): Path<String>,
) -> impl IntoResponse {
    handle_get_public_key(use_cases, Some(key_id)).await
}

async fn handle_get_public_key(use_cases: UseCases, key_id: Option<String>) -> impl IntoResponse {
    let result = use_cases
        .get_public_key(GetPublicKeyRequest {
            key_id: key_id.as_deref(),
        })
        .await;

    match result {
        Ok(response) => ProtoResponse::new(
            StatusCode::OK,
            GetPublicKeyResponseProto {
                result: Some(Result::Success(GetPublicKeySuccessResponseProto {
                    public_key_pem: response.public_key_pem,
                })),
            },
        ),
        Err(GetPublicKeyError::KeyPairNotFound) => ProtoResponse::new(
            StatusCode::NOT_FOUND,
            GetPublicKeyResponseProto {
                result: Some(Result::Error(
                    GetPublicKeyErrorCodeProto::KeypairNotFound.into(),
                )),
            },
        ),
        Err(GetPublicKeyError::KeyPairIsExpired | GetPublicKeyError::KeyPairIsRevoked) => {
            ProtoResponse::new(
                StatusCode::BAD_REQUEST,
                GetPublicKeyResponseProto {
                    result: Some(Result::Error(
                        GetPublicKeyErrorCodeProto::InvalidKeypairRequested.into(),
                    )),
                },
            )
        }
        Err(err) => {
            error!("error in handle_get_public_key handler: {}", err);
            ProtoResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                GetPublicKeyResponseProto {
                    result: Some(Result::Error(GetPublicKeyErrorCodeProto::Undefined.into())),
                },
            )
        }
    }
}
