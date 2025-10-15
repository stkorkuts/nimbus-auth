use axum::{
    Router,
    http::{HeaderName, HeaderValue},
};
use tower_http::set_header::SetResponseHeaderLayer;

const HSTS_HEADER_NAME: &str = "strict-transport-security";
const HSTS_HEADER_VALUE: &str = "max-age=31536000; includeSubDomains; preload";

pub fn apply_hsts_middleware(router: Router) -> Router {
    let hsts_layer = SetResponseHeaderLayer::if_not_present(
        HeaderName::from_static(HSTS_HEADER_NAME),
        HeaderValue::from_static(HSTS_HEADER_VALUE),
    );
    router.layer(hsts_layer)
}
