use crate::db::models::{CreateUser, User};
use axum::http::StatusCode;
use axum::{Json, extract::Extension};
use bcrypt::{DEFAULT_COST, hash};
use sqlx::{Error as SqlxError, Pool, Postgres};
use tracing;

// Helper function to hash passwords using bcrypt
fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

pub async fn create_user(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(new_user): Json<CreateUser>,
) -> Result<Json<User>, (StatusCode, Json<serde_json::Value>)> {
    // Validate input
    if new_user.name.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Name cannot be empty"
            })),
        ));
    }

    if new_user.email.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Email cannot be empty"
            })),
        ));
    }

    if new_user.password.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Password cannot be empty"
            })),
        ));
    }

    // Hash the password
    let password_hash = match hash_password(&new_user.password) {
        Ok(hash) => hash,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to hash password"
                })),
            ));
        }
    };

    // Attempt to create user
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (name, email, password_hash) VALUES ($1, $2, $3) RETURNING id, name, email, password_hash, created_at, updated_at",
        new_user.name.trim(),
        new_user.email.trim().to_lowercase(),
        password_hash
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        match e {
            SqlxError::Database(db_error) => {
                if let Some(code) = db_error.code() {
                    // PostgreSQL unique constraint violation
                    if code.as_ref() == "23505" {
                        return (
                            StatusCode::CONFLICT,
                            Json(serde_json::json!({
                                "error": "Email already exists"
                            }))
                        );
                    }
                }
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "error": "Database error occurred"
                    }))
                )
            }
            SqlxError::RowNotFound => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to create user"
                }))
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "An unexpected error occurred"
                }))
            )
        }
    })?;

    tracing::info!("Successfully created user with email: {}", user.email);
    Ok(Json(user))
}

pub async fn retrieve_user_by_email(
    pool: &Pool<Postgres>,
    email: &str,
) -> Result<Option<User>, SqlxError> {
    tracing::debug!("Looking up user by email: {}", email);

    let user = sqlx::query_as!(
        User,
        "SELECT id, name, email, password_hash, created_at, updated_at FROM users WHERE email = $1",
        email.trim().to_lowercase()
    )
    .fetch_optional(pool)
    .await?;

    match &user {
        Some(user) => {
            tracing::info!("User found in database for email: {}", email);
        }
        None => {
            tracing::warn!("User not found in database for email: {}", email);
        }
    }

    Ok(user)
}
