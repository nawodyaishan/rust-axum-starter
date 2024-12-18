use axum::routing::{get, post};
use axum::Router;

use crate::handlers::v1::{create_user, get_users_handler};

/// Defines the routes for API version v1.
pub fn router() -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route("/users", get(get_users_handler))
}
