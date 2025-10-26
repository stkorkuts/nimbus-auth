use axum::{
    body::Bytes,
    http::{
        HeaderMap, HeaderName, HeaderValue, StatusCode,
        header::{CONTENT_TYPE, InvalidHeaderValue},
    },
    response::{IntoResponse, Response},
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use nimbus_auth_application::use_cases::SessionDto;
use prost::Message;

use crate::web_api::extractors::client_extractor::ClientType;

pub struct ProtoResponse<T: Message> {
    message: T,
    status_code: StatusCode,
    extra_headers: HeaderMap,
    cookie_jar: Option<CookieJar>,
}

impl<T: Message> ProtoResponse<T> {
    pub fn new(status_code: StatusCode, message: T) -> Self {
        Self {
            message,
            status_code,
            extra_headers: HeaderMap::new(),
            cookie_jar: Some(CookieJar::new()),
        }
    }

    fn set_header(&mut self, name: HeaderName, value: HeaderValue) -> &mut Self {
        self.extra_headers.insert(name, value);
        self
    }

    fn set_cookie(&mut self, cookie: impl Into<Cookie<'static>>) -> &mut Self {
        self.cookie_jar = Some(
            self.cookie_jar
                .take()
                .expect("cookie jar should always be some")
                .add(cookie),
        );
        self
    }

    pub fn with_session_headers(
        mut self,
        client_type: ClientType,
        session: &SessionDto,
    ) -> Result<Self, InvalidHeaderValue> {
        match client_type {
            ClientType::Browser => {
                self.set_cookie(
                    Cookie::build(("session_id", session.session_id.to_string()))
                        .http_only(true)
                        .secure(true)
                        .same_site(SameSite::Strict),
                )
                .set_cookie(
                    Cookie::build((
                        "session_exp_timestamp",
                        session.session_expires_at_unix_timestamp.to_string(),
                    ))
                    .http_only(true)
                    .secure(true)
                    .same_site(SameSite::Strict),
                );
            }
            _ => {
                self.set_header(
                    HeaderName::from_static("x-session-id"),
                    HeaderValue::from_str(&session.session_id)?,
                )
                .set_header(
                    HeaderName::from_static("x-session-exp-timestamp"),
                    HeaderValue::from_str(&session.session_expires_at_unix_timestamp.to_string())?,
                );
            }
        }
        Ok(self)
    }
}

impl<T: Message> IntoResponse for ProtoResponse<T> {
    fn into_response(mut self) -> Response {
        let bytes = self.message.encode_to_vec();
        self.extra_headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-protobuf"),
        );
        (
            self.status_code,
            self.extra_headers,
            self.cookie_jar,
            Bytes::from(bytes),
        )
            .into_response()
    }
}
