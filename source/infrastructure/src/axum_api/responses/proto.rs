use axum::{
    body::Bytes,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use prost::Message;

pub struct ProtoResponse<T: Message> {
    message: T,
    status_code: StatusCode,
}

impl<T: Message> ProtoResponse<T> {
    pub fn new(status_code: StatusCode, message: T) -> Self {
        Self {
            message,
            status_code,
        }
    }
}

impl<T: Message> IntoResponse for ProtoResponse<T> {
    fn into_response(self) -> Response {
        let bytes = self.message.encode_to_vec();
        let mut headers = HeaderMap::new();
        headers.insert(
            "content-type",
            HeaderValue::from_static("application/x-protobuf"),
        );
        (self.status_code, headers, Bytes::from(bytes)).into_response()
    }
}
