use axum::{body::Bytes, extract::State, http::HeaderMap, response::IntoResponse};
use nimbus_auth_application::use_cases::UseCases;

pub async fn handle_signup(
    State(use_cases): State<UseCases>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
}
