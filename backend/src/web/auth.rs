use axum::{
    body::Body,
    extract::{Extension, Json, Request},
    http,
    http::{Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{Pool, Postgres};
use tracing::{debug, error, info, warn};

use crate::db::models::{SignInData, User};
use crate::db::user_queries;

#[derive(Serialize, Deserialize)]
// Define a structure for holding claims data used in JWT tokens
pub struct Claims {
    pub exp: usize,    // Expiry time of the token
    pub iat: usize,    // Issued at time of the token
    pub email: String, // Email associated with the token
}

#[derive(Serialize, Deserialize)]
pub struct SignInResponse {
    pub success: bool,
    pub message: String,
    pub token: String,
}

#[derive(Debug)]
pub struct AuthError {
    pub message: String,
    pub status_code: StatusCode,
}
// Function to handle sign-in requests with comprehensive error handling
pub async fn sign_in_handler(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(user_data): Json<SignInData>, // JSON payload containing sign-in data
) -> Result<Json<SignInResponse>, StatusCode> {
    // Return type is a JSON-wrapped SignInResponse or an HTTP status code

    info!("Sign-in attempt received for email: {}", user_data.email);
    debug!("Sign-in request payload: {:?}", user_data);

    // Validate input data
    if user_data.email.is_empty() {
        error!("Sign-in failed: Empty email provided");
        return Err(StatusCode::BAD_REQUEST);
    }

    if user_data.password.is_empty() {
        error!("Sign-in failed: Empty password provided");
        return Err(StatusCode::BAD_REQUEST);
    }

    // Attempt to retrieve user information based on the provided email
    info!("Attempting to retrieve user by email: {}", user_data.email);
    let user = match user_queries::retrieve_user_by_email(&pool, &user_data.email).await {
        Ok(Some(user)) => {
            info!("User found for email: {}", user_data.email);
            user // User found, proceed with authentication
        }
        Ok(None) => {
            error!(
                "Sign-in failed: User not found for email: {}",
                user_data.email
            );
            return Err(StatusCode::UNAUTHORIZED); // User not found, return unauthorized status
        }
        Err(e) => {
            error!(
                "Sign-in failed: Database error while retrieving user {}: {:?}",
                user_data.email, e
            );
            return Err(StatusCode::INTERNAL_SERVER_ERROR); // Database error
        }
    };

    // Verify the password provided against the stored hash
    info!("Verifying password for user: {}", user_data.email);
    let password_verification = verify_password(&user_data.password, &user.password_hash);
    match password_verification {
        Ok(is_valid) => {
            if is_valid {
                info!(
                    "Password verification successful for user: {}",
                    user_data.email
                );
            } else {
                error!(
                    "Sign-in failed: Invalid password for user: {}",
                    user_data.email
                );
                return Err(StatusCode::UNAUTHORIZED); // Password verification failed, return unauthorized status
            }
        }
        Err(e) => {
            error!(
                "Sign-in failed: Password verification error for user {}: {:?}",
                user_data.email, e
            );
            return Err(StatusCode::INTERNAL_SERVER_ERROR); // Handle bcrypt errors
        }
    }

    // Generate a JWT token for the authenticated user
    info!("Generating JWT token for user: {}", user_data.email);
    let token = match encode_jwt(user.email.clone()) {
        Ok(token) => {
            info!(
                "JWT token generated successfully for user: {}",
                user_data.email
            );
            token
        }
        Err(e) => {
            error!(
                "Sign-in failed: JWT encoding error for user {}: {:?}",
                user_data.email, e
            );
            return Err(StatusCode::INTERNAL_SERVER_ERROR); // Handle JWT encoding errors
        }
    };

    info!("Sign-in successful for user: {}", user_data.email);
    // Return the response as a JSON object with success, message, and token
    Ok(Json(SignInResponse {
        success: true,
        message: "Login successful".to_string(),
        token,
    }))
}

// Legacy function name for backward compatibility
pub async fn sign_in(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(user_data): Json<SignInData>,
) -> Result<Json<SignInResponse>, StatusCode> {
    sign_in_handler(Extension(pool), Json(user_data)).await
}

pub fn encode_jwt(email: String) -> Result<String, StatusCode> {
    debug!("Encoding JWT token for email: {}", email);

    let secret = std::env::var("JWT_SECRET").map_err(|_| {
        error!("JWT_SECRET environment variable not set");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::hours(24);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;
    let claim = Claims { iat, exp, email };

    let result = encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    );

    match result {
        Ok(token) => {
            debug!("JWT token encoded successfully");
            Ok(token)
        }
        Err(e) => {
            error!("JWT encoding failed: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub fn decode_jwt(jwt_token: String) -> Result<TokenData<Claims>, StatusCode> {
    let secret = std::env::var("JWT_SECRET").map_err(|_| {
        error!("JWT_SECRET environment variable not set");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let result: Result<TokenData<Claims>, StatusCode> = decode(
        &jwt_token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    result
}

pub async fn authorization_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);
    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_| StatusCode::FORBIDDEN)?,
        None => {
            return Err(StatusCode::FORBIDDEN);
        }
    };
    let mut header = auth_header.split_whitespace();
    let (bearer, token) = (header.next(), header.next());
    let token_data = match decode_jwt(token.unwrap().to_string()) {
        Ok(data) => data,
        Err(_) => {
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // Get the database pool from request extensions
    let pool = req
        .extensions()
        .get::<Pool<Postgres>>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    // Fetch the user details from the database
    let current_user =
        match user_queries::retrieve_user_by_email(pool, &token_data.claims.email).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                return Err(StatusCode::UNAUTHORIZED);
            }
            Err(_) => {
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
    req.extensions_mut().insert(current_user);
    Ok(next.run(req).await)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}
