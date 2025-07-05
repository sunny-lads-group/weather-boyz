use axum::{Router, routing::post};
use tower_http::cors::CorsLayer;

mod db;
use db::user_queries::create_user;

mod blockchain;
mod web;

#[cfg(test)]
mod test_utils;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenvy::from_path("../.env").ok();
    let backend_url: String =
        std::env::var("BACKEND_URL").unwrap_or_else(|_| "http://0.0.0.0:6969".to_string());
    
    let backend_address: String =
        std::env::var("BACKEND_ADDRESS").unwrap_or_else(|_| "0.0.0.0:6969".to_string());


    tracing_subscriber::fmt::init();
    tracing::info!("Starting weather-boyz backend server...");

    // Test the database connection
    tracing::info!("Testing database connection...");
    let pool = match db::pool::get_pool().await {
        Ok(pool) => {
            tracing::info!("Database connection successful");
            pool
        }
        Err(e) => {
            tracing::error!("Failed to connect to database: {:?}", e);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::permissive();
    tracing::info!("CORS layer configured");

    // Create your main router
    tracing::info!("Setting up main router...");
    let main_router = Router::new().route("/createUser", post(create_user));

    // Get the web routes router
    tracing::info!("Setting up web routes...");
    let web_router = web::routes::app().await;

    // Merge the routers
    let app = main_router
        .merge(web_router)
        .layer(cors)
        .layer(axum::extract::Extension(pool));

    let listener = match tokio::net::TcpListener::bind(&backend_address).await {
        Ok(listener) => {
            tracing::info!("Server bound to {}", backend_address);
            listener
        }
        Err(e) => {
            tracing::error!("Failed to bind to {}: {:?}", backend_address, e);
            std::process::exit(1);
        }
    };

    tracing::info!("Server running on {}", backend_url);
    tracing::info!("Available endpoints:");
    tracing::info!("  POST /signin - User Authentication (get token)");
    tracing::info!("  POST /createUser - Create new user");
    tracing::info!("  GET  /tokenvalid/ - Protected route (requires token auth)");

    if let Err(e) = axum::serve(listener, app).await {
        tracing::error!("Server error: {:?}", e);
        std::process::exit(1);
    }
}
