use axum::{
    extract::FromRequestParts,
    http::{HeaderMap, StatusCode},
};
use axum_extra::extract::CookieJar;
use nimbus_auth_application::use_cases::UseCases;
use nimbus_auth_shared::constants::{SESSION_COOKIE_NAME, SESSION_HEADER_NAME};

use crate::web_api::extractors::client_extractor::{Client, ClientType};

pub struct Session {
    pub session_id: String,
}

impl FromRequestParts<UseCases> for Session {
    type Rejection = (StatusCode, &'static str);

    fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &UseCases,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async {
            let client = Client::from_request_parts(parts, state).await?;
            Ok(Session {
                session_id: Self::extract_session_id(client, &parts.headers)?,
            })
        }
    }
}

impl Session {
    fn extract_session_id(
        client: Client,
        headers: &HeaderMap,
    ) -> Result<String, <Session as FromRequestParts<UseCases>>::Rejection> {
        match client.0 {
            ClientType::Browser => {
                let cookies = CookieJar::from_headers(headers);
                let session_cookie = cookies
                    .get(SESSION_COOKIE_NAME)
                    .ok_or((StatusCode::BAD_REQUEST, "session cookie is not found"))?;
                Ok(session_cookie.value().to_string())
            }
            _ => {
                let session_header = headers
                    .get(SESSION_HEADER_NAME)
                    .ok_or((StatusCode::BAD_REQUEST, "session header is not found"))?;
                Ok(session_header
                    .to_str()
                    .map_err(|_| (StatusCode::BAD_REQUEST, "session header is invalid"))?
                    .to_string())
            }
        }
    }
}
