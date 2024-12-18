use axum::Router;

pub mod v1;
pub mod v2;

/// Combines all routes into a single router.
pub fn app_router() -> Router {
    Router::new()
        .nest("/v1", v1::router())
        .nest("/v2", v2::router())
}
