use axum::Router;
use tower_http::trace::TraceLayer;

pub fn apply_tracing_middleware(router: Router) -> Router {
    router.layer(TraceLayer::new_for_http())
}
