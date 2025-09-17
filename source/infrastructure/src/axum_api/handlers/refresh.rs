use axum::{body::Bytes, extract::State, http::HeaderMap};
use nimbus_auth_application::use_cases::UseCases;

pub async fn handle_refresh(State(use_cases): State<UseCases>, headers: HeaderMap, body: Bytes) {}
