use axum::{Extension, Json, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::db::models::User;
use crate::web::{
    auth::{self},
    services,
};
#[derive(Serialize, Deserialize)]
struct UserResponse {
    email: String,
    name: String,
}

pub async fn hello(Extension(current_user): Extension<User>) -> impl IntoResponse {
    Json(UserResponse {
        email: current_user.email,
        name: current_user.name,
    })
}
