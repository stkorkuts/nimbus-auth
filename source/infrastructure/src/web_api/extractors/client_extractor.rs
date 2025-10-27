use axum::{
    extract::FromRequestParts,
    http::{HeaderMap, StatusCode, request::Parts},
};
use nimbus_auth_application::use_cases::UseCases;

pub enum ClientType {
    Browser,
    Mobile,
    PC,
}

pub struct Client(pub ClientType);

impl FromRequestParts<UseCases> for Client {
    type Rejection = (StatusCode, &'static str);

    fn from_request_parts(
        parts: &mut Parts,
        _: &UseCases,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async { Ok(Client(Self::detect_client_type(&parts.headers)?)) }
    }
}

impl Client {
    fn detect_client_type(
        headers: &HeaderMap,
    ) -> Result<ClientType, <Client as FromRequestParts<UseCases>>::Rejection> {
        if let Some(header_value) = headers
            .get("x-client-type")
            .and_then(|header_value| header_value.to_str().ok())
        {
            return match header_value.to_ascii_lowercase().as_str() {
                "browser" => Ok(ClientType::Browser),
                "mobile" => Ok(ClientType::Mobile),
                "pc" => Ok(ClientType::PC),
                _ => Err((StatusCode::BAD_REQUEST, "client type header is not found")),
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
                return Ok(ClientType::Mobile);
            }

            if ua_lc.contains("mozilla") || ua_lc.contains("chrome") || ua_lc.contains("safari") {
                return Ok(ClientType::Browser);
            }

            if ua_lc.contains("windows") || ua_lc.contains("macintosh") || ua_lc.contains("linux") {
                return Ok(ClientType::PC);
            }
        }

        Err((StatusCode::BAD_REQUEST, "client type header is not found"))
    }
}
