use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde::Deserialize;

use crate::db::{add_user, get_users};
use crate::models::user::User;

/// Request payload for creating a new user.
#[derive(Debug, Deserialize)]
pub struct CreateUserPayload {
    pub name: String,
    pub email: String,
}

/// Handler to create a new user (v2).
pub async fn create_user(Json(payload): Json<CreateUserPayload>) -> impl IntoResponse {
    // In v2, we might add additional processing or validation
    let user = User::new(payload.name, payload.email);

    match add_user(user.clone()) {
        Ok(_) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user").into_response(),
    }
}

/// Handler to get all users (v2).
pub async fn get_users_handler() -> impl IntoResponse {
    let users = get_users();
    // In v2, we might filter or modify the response
    (StatusCode::OK, Json(users)).into_response()
}
