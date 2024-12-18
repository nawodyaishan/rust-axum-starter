use axum::routing::{get, post};
use axum::Router;

use crate::handlers::v2::{create_user, get_users_handler};

/// Defines the routes for API version v2.
pub fn router() -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route("/users", get(get_users_handler))
}
