use axum::{extract::State, http::StatusCode, response::IntoResponse};
use nimbus_auth_application::use_cases::{RotateKeyPairsError, RotateKeyPairsRequest, UseCases};
use nimbus_auth_proto::proto::{
    RotateKeypairsErrorCodeProto, RotateKeypairsResponseProto, RotateKeypairsSuccessResponseProto,
    rotate_keypairs_response_proto::Result,
};
use tracing::error;

use crate::web_api::{
    extractors::authorization_extractor::Authorization, responses::proto::ProtoResponse,
};

pub async fn handle_rotate_keypairs(
    State(use_cases): State<UseCases>,
    Authorization(user): Authorization,
) -> impl IntoResponse {
    let result = use_cases
        .rotate_keypairs(RotateKeyPairsRequest { user })
        .await;

    match result {
        Ok(_) => ProtoResponse::new(
            StatusCode::OK,
            RotateKeypairsResponseProto {
                result: Some(Result::Success(RotateKeypairsSuccessResponseProto {})),
            },
        ),
        Err(RotateKeyPairsError::Forbidden(_)) => ProtoResponse::new(
            StatusCode::FORBIDDEN,
            RotateKeypairsResponseProto {
                result: Some(Result::Error(
                    RotateKeypairsErrorCodeProto::Forbidden.into(),
                )),
            },
        ),
        Err(err) => {
            error!("error in handle_rotate_keypairs handler: {err}");
            ProtoResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                RotateKeypairsResponseProto {
                    result: Some(Result::Error(
                        RotateKeypairsErrorCodeProto::Undefined.into(),
                    )),
                },
            )
        }
    }
}
