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
    Authorization(auth_result): Authorization,
) -> impl IntoResponse {
    let user = match auth_result {
        Ok(user) => user,
        Err(err) => {
            error!("can not extract authenticated user in handle_rotate_keypairs. error: {err}");
            return ProtoResponse::new(
                StatusCode::UNAUTHORIZED,
                RotateKeypairsResponseProto {
                    result: Some(Result::Error(
                        RotateKeypairsErrorCodeProto::Unauthorized.into(),
                    )),
                },
            );
        }
    };

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
            error!("error while rotating keypairs: {err}");
            ProtoResponse::new(
                StatusCode::BAD_REQUEST,
                RotateKeypairsResponseProto {
                    result: Some(Result::Error(
                        RotateKeypairsErrorCodeProto::Undefined.into(),
                    )),
                },
            )
        }
    }
}
