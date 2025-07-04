use axum::{Router, routing::post};
use tower_http::cors::CorsLayer;

mod db;
use db::user_queries::create_user;

mod web;
mod blockchain;

#[cfg(test)]
mod test_utils;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

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

    let listener = match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
        Ok(listener) => {
            tracing::info!("Server bound to 0.0.0.0:3000");
            listener
        }
        Err(e) => {
            tracing::error!("Failed to bind to 0.0.0.0:3000: {:?}", e);
            std::process::exit(1);
        }
    };

    tracing::info!("Server running on http://0.0.0.0:3000");
    tracing::info!("Available endpoints:");
    tracing::info!("  POST /signin - User Authentication (get token)");
    tracing::info!("  POST /createUser - Create new user");
    tracing::info!("  GET  /tokenvalid/ - Protected route (requires token auth)");

    if let Err(e) = axum::serve(listener, app).await {
        tracing::error!("Server error: {:?}", e);
        std::process::exit(1);
    }
}
