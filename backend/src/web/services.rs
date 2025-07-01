use axum::{Extension, Json, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::Pool;
use sqlx::Postgres;

use crate::db::models::{User, PolicyTemplate};
use crate::db::policy_queries;
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

pub async fn get_policy_templates(
    Extension(pool): Extension<Pool<Postgres>>,
    Extension(current_user): Extension<User>,
) -> impl IntoResponse {
    match policy_queries::get_all_policy_templates(&pool).await {
        Ok(templates) => {
            tracing::info!("Retrieved {} policy templates for user {}", templates.len(), current_user.email);
            Json(templates).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to fetch policy templates: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch policy templates").into_response()
        }
    }
}
