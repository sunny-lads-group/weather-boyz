#[cfg(test)]
use std::env;
use sqlx::{Pool, Postgres as SqlxPostgres, PgPool};
use axum::Router;
use tower_http::cors::CorsLayer;
use testcontainers::{ContainerAsync, runners::AsyncRunner};
use testcontainers_modules::postgres::Postgres;

use crate::db::models::{CreateUser, User};
use crate::db::user_queries;
use crate::web;

pub struct TestDatabase {
    pub pool: Pool<SqlxPostgres>,
    pub _container: ContainerAsync<Postgres>,
}

/// Create a test PostgreSQL database using testcontainers
pub async fn create_test_db() -> TestDatabase {
    let postgres_image = Postgres::default();
    
    let container = postgres_image
        .start()
        .await
        .expect("Failed to start PostgreSQL container");
    
    let port = container
        .get_host_port_ipv4(5432)
        .await
        .expect("Failed to get container port");
    
    let database_url = format!(
        "postgres://postgres:postgres@localhost:{}/postgres",
        port
    );
    
    // Create connection pool
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");
    
    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations on test database");
    
    TestDatabase {
        pool,
        _container: container,
    }
}

/// Create a test Axum app with test PostgreSQL database
pub async fn create_test_app() -> (Router, TestDatabase) {
    // Set up test environment variables
    unsafe {
        env::set_var("JWT_SECRET", "test_jwt_secret_key_for_integration_tests");
    }

    let test_db = create_test_db().await;
    let cors = CorsLayer::permissive();
    let pool_extension = axum::extract::Extension(test_db.pool.clone());
    
    // Create your main router with database extension
    let main_router = Router::new()
        .route("/createUser", axum::routing::post(user_queries::create_user))
        .layer(pool_extension.clone());

    // Get the web routes router with database extension
    let web_router = web::routes::app().await
        .layer(pool_extension.clone());

    // Merge the routers
    let app = main_router
        .merge(web_router)
        .layer(cors);
        
    (app, test_db)
}

/// Helper function to create a test user in the database
pub async fn create_test_user(
    pool: &Pool<SqlxPostgres>,
    name: &str,
    email: &str,
    password: &str,
) -> Result<User, sqlx::Error> {
    let create_user = CreateUser {
        name: name.to_string(),
        email: email.to_string(),
        password: password.to_string(),
    };

    // Hash the password
    let password_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|_| sqlx::Error::Protocol("Failed to hash password".into()))?;

    // Insert user into PostgreSQL database using your existing query
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (name, email, password_hash) VALUES ($1, $2, $3) RETURNING id, name, email, password_hash, wallet_address, created_at, updated_at",
        create_user.name.trim(),
        create_user.email.trim().to_lowercase(),
        password_hash
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

/// Helper function to generate a valid JWT token for testing
pub fn create_test_jwt(email: &str) -> String {
    use crate::web::auth::encode_jwt;
    
    unsafe {
        env::set_var("JWT_SECRET", "test_jwt_secret_key_for_integration_tests");
    }
    encode_jwt(email.to_string()).expect("Failed to create test JWT")
}

/// Clean up test environment variables
pub fn cleanup_test_env() {
    unsafe {
        env::remove_var("JWT_SECRET");
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use axum_test::TestServer;
    use serde_json;
    use axum::http;

    #[tokio::test]
    async fn test_create_user_success() {
        let (app, _test_db) = create_test_app().await;
        let server = TestServer::new(app).unwrap();

        let response = server
            .post("/createUser")
            .json(&serde_json::json!({
                "name": "Test User",
                "email": "test@example.com",
                "password": "password123"
            }))
            .await;

        response.assert_status_ok();
        
        let user: User = response.json();
        assert_eq!(user.name, "Test User");
        assert_eq!(user.email, "test@example.com");
        assert!(user.id > 0);
        
        cleanup_test_env();
    }

    #[tokio::test]
    async fn test_create_user_duplicate_email() {
        let (app, test_db) = create_test_app().await;
        let server = TestServer::new(app).unwrap();

        // Create first user
        create_test_user(&test_db.pool, "First User", "duplicate@example.com", "password123")
            .await
            .expect("Failed to create first test user");

        // Try to create second user with same email
        let response = server
            .post("/createUser")
            .json(&serde_json::json!({
                "name": "Second User",
                "email": "duplicate@example.com",
                "password": "password456"
            }))
            .await;

        response.assert_status(axum::http::StatusCode::CONFLICT);
        
        cleanup_test_env();
    }

    #[tokio::test]
    async fn test_create_user_missing_fields() {
        let (app, _test_db) = create_test_app().await;
        let server = TestServer::new(app).unwrap();

        let response = server
            .post("/createUser")
            .json(&serde_json::json!({
                "name": "Test User"
                // Missing email and password
            }))
            .await;

        response.assert_status(axum::http::StatusCode::UNPROCESSABLE_ENTITY);
        
        cleanup_test_env();
    }

    #[tokio::test]
    async fn test_signin_success() {
        let (app, test_db) = create_test_app().await;
        let server = TestServer::new(app).unwrap();

        // Create a test user first
        create_test_user(&test_db.pool, "Login User", "login@example.com", "password123")
            .await
            .expect("Failed to create test user");

        let response = server
            .post("/signin")
            .json(&serde_json::json!({
                "email": "login@example.com",
                "password": "password123"
            }))
            .await;

        response.assert_status_ok();
        
        let signin_response: serde_json::Value = response.json();
        assert_eq!(signin_response["success"], true);
        assert!(signin_response["token"].is_string());
        assert!(!signin_response["token"].as_str().unwrap().is_empty());
        
        cleanup_test_env();
    }

    #[tokio::test]
    async fn test_signin_invalid_credentials() {
        let (app, test_db) = create_test_app().await;
        let server = TestServer::new(app).unwrap();

        // Create a test user first
        create_test_user(&test_db.pool, "Auth User", "auth@example.com", "correct_password")
            .await
            .expect("Failed to create test user");

        let response = server
            .post("/signin")
            .json(&serde_json::json!({
                "email": "auth@example.com",
                "password": "wrong_password"
            }))
            .await;

        response.assert_status(axum::http::StatusCode::UNAUTHORIZED);
        
        cleanup_test_env();
    }

    #[tokio::test]
    async fn test_signin_user_not_found() {
        let (app, _test_db) = create_test_app().await;
        let server = TestServer::new(app).unwrap();

        let response = server
            .post("/signin")
            .json(&serde_json::json!({
                "email": "nonexistent@example.com",
                "password": "any_password"
            }))
            .await;

        response.assert_status(axum::http::StatusCode::UNAUTHORIZED);
        
        cleanup_test_env();
    }

    #[tokio::test]
    async fn test_protected_route_with_valid_token() {
        let (app, test_db) = create_test_app().await;
        let server = TestServer::new(app).unwrap();

        // Create a test user
        let user = create_test_user(&test_db.pool, "Token User", "token@example.com", "password123")
            .await
            .expect("Failed to create test user");

        // Generate a valid JWT token
        let token = create_test_jwt(&user.email);

        let response = server
            .get("/tokenvalid/")
            .add_header(
                http::header::AUTHORIZATION,
                format!("Bearer {}", token)
            )
            .await;

        response.assert_status_ok();
        
        let user_response: serde_json::Value = response.json();
        assert_eq!(user_response["email"], "token@example.com");
        assert_eq!(user_response["name"], "Token User");
        
        cleanup_test_env();
    }

    #[tokio::test]
    async fn test_protected_route_without_token() {
        let (app, _test_db) = create_test_app().await;
        let server = TestServer::new(app).unwrap();

        let response = server
            .get("/tokenvalid/")
            .await;

        response.assert_status(axum::http::StatusCode::FORBIDDEN);
        
        cleanup_test_env();
    }

    #[tokio::test]
    async fn test_protected_route_with_invalid_token() {
        let (app, _test_db) = create_test_app().await;
        let server = TestServer::new(app).unwrap();

        let response = server
            .get("/tokenvalid/")
            .add_header(
                http::header::AUTHORIZATION,
                "Bearer invalid_token_here"
            )
            .await;

        response.assert_status(axum::http::StatusCode::UNAUTHORIZED);
        
        cleanup_test_env();
    }

    #[tokio::test]
    async fn test_complete_user_journey() {
        let (app, _test_db) = create_test_app().await;
        let server = TestServer::new(app).unwrap();

        // Step 1: Register a new user
        let create_response = server
            .post("/createUser")
            .json(&serde_json::json!({
                "name": "Journey User",
                "email": "journey@example.com",
                "password": "journey_password123"
            }))
            .await;

        create_response.assert_status_ok();

        // Step 2: Sign in with the new user
        let signin_response = server
            .post("/signin")
            .json(&serde_json::json!({
                "email": "journey@example.com",
                "password": "journey_password123"
            }))
            .await;

        signin_response.assert_status_ok();
        let signin_data: serde_json::Value = signin_response.json();
        let token = signin_data["token"].as_str().expect("Expected token in response");

        // Step 3: Access protected route with the token
        let protected_response = server
            .get("/tokenvalid/")
            .add_header(
                http::header::AUTHORIZATION,
                format!("Bearer {}", token)
            )
            .await;

        protected_response.assert_status_ok();
        let user_data: serde_json::Value = protected_response.json();
        assert_eq!(user_data["email"], "journey@example.com");
        assert_eq!(user_data["name"], "Journey User");
        
        cleanup_test_env();
    }
}