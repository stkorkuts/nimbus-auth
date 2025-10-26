use axum::{
    extract::FromRequestParts,
    http::{HeaderMap, StatusCode, request::Parts},
};

pub mod errors;

pub enum ClientType {
    Undefined,
    Browser,
    Mobile,
    PC,
}

pub struct Client(pub ClientType);

impl<S> FromRequestParts<S> for Client {
    type Rejection = (StatusCode, &'static str);

    fn from_request_parts(
        parts: &mut Parts,
        _: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async { Ok(Client(detect_client_type(&parts.headers))) }
    }
}

fn detect_client_type(headers: &HeaderMap) -> ClientType {
    if let Some(header_value) = headers
        .get("x-client-type")
        .and_then(|header_value| header_value.to_str().ok())
    {
        return match header_value.to_ascii_lowercase().as_str() {
            "browser" => ClientType::Browser,
            "mobile" => ClientType::Mobile,
            "pc" => ClientType::PC,
            _ => ClientType::Undefined,
        };
    }

    if let Some(user_agent) = headers
        .get("user-agent")
        .and_then(|header_value| header_value.to_str().ok())
    {
        let ua_lc = user_agent.to_ascii_lowercase();

        if ua_lc.contains("android")
            || ua_lc.contains("iphone")
            || ua_lc.contains("ipad")
            || ua_lc.contains("mobile")
        {
            return ClientType::Mobile;
        }

        if ua_lc.contains("mozilla") || ua_lc.contains("chrome") || ua_lc.contains("safari") {
            return ClientType::Browser;
        }

        if ua_lc.contains("windows") || ua_lc.contains("macintosh") || ua_lc.contains("linux") {
            return ClientType::PC;
        }
    }

    ClientType::Undefined
}
