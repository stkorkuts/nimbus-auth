use axum::{extract::State, http::HeaderMap, response::IntoResponse};
use nimbus_auth_application::use_cases::UseCases;

pub async fn handle_get_public_key(State(use_cases): State<UseCases>, headers: HeaderMap) {
    todo!();
}
