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
        "INSERT INTO users (name, email, password_hash) VALUES ($1, $2, $3) RETURNING id, name, email, password_hash, wallet_address, created_at, updated_at",
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
        "SELECT id, name, email, password_hash, wallet_address, created_at, updated_at FROM users WHERE email = $1",
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

pub async fn update_user_wallet_address(
    pool: &Pool<Postgres>,
    user_id: i32,
    wallet_address: &str,
) -> Result<User, SqlxError> {
    tracing::info!("Updating wallet address for user id: {}", user_id);

    let user = sqlx::query_as!(
        User,
        "UPDATE users SET wallet_address = $1, updated_at = CURRENT_TIMESTAMP 
         WHERE id = $2 
         RETURNING id, name, email, password_hash, wallet_address, created_at, updated_at",
        wallet_address,
        user_id
    )
    .fetch_one(pool)
    .await?;

    tracing::info!("Successfully updated wallet address for user id: {}", user_id);
    Ok(user)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password_success() {
        let password = "test_password_123";
        let result = hash_password(password);
        
        assert!(result.is_ok());
        let hash = result.unwrap();
        assert!(!hash.is_empty());
        assert_ne!(hash, password); // Hash should be different from original password
        
        // Verify the hash starts with bcrypt identifier
        assert!(hash.starts_with("$2b$") || hash.starts_with("$2a$") || hash.starts_with("$2y$"));
    }

    #[test]
    fn test_hash_password_different_results() {
        let password = "same_password";
        let hash1 = hash_password(password).unwrap();
        let hash2 = hash_password(password).unwrap();
        
        // Different hashes should be generated due to salt
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_hash_password_empty_string() {
        let empty_password = "";
        let result = hash_password(empty_password);
        
        // Should handle empty passwords
        assert!(result.is_ok());
        let hash = result.unwrap();
        assert!(!hash.is_empty());
    }

    #[test]
    fn test_hash_password_long_string() {
        // Test with a very long password
        let long_password = "a".repeat(1000);
        let result = hash_password(&long_password);
        
        // Should handle long passwords
        assert!(result.is_ok());
        let hash = result.unwrap();
        assert!(!hash.is_empty());
    }

    #[test]
    fn test_hash_password_special_characters() {
        let special_password = "p@ssw0rd!#$%^&*()";
        let result = hash_password(special_password);
        
        assert!(result.is_ok());
        let hash = result.unwrap();
        assert!(!hash.is_empty());
    }

    #[test]
    fn test_hash_password_unicode_characters() {
        let unicode_password = "pàssw🔑rd";
        let result = hash_password(unicode_password);
        
        assert!(result.is_ok());
        let hash = result.unwrap();
        assert!(!hash.is_empty());
    }
}
