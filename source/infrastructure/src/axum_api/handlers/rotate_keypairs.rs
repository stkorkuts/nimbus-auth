use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use nimbus_auth_application::use_cases::UseCases;
use nimbus_auth_proto::proto::{
    RotateKeypairsErrorCodeProto, RotateKeypairsResponseProto, RotateKeypairsSuccessResponseProto,
    rotate_keypairs_response_proto::Result,
};
use tracing::error;

use crate::axum_api::{
    extractors::authorization_extractor::Authorization, responses::proto::ProtoResponse,
};

pub async fn handle_rotate_keypairs(
    State(use_cases): State<UseCases>,
    Authorization(auth_result): Authorization,
    headers: HeaderMap,
    body: Bytes,
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

    

    ProtoResponse::new(
        StatusCode::OK,
        RotateKeypairsResponseProto {
            result: Some(Result::Success(RotateKeypairsSuccessResponseProto {})),
        },
    )
}
