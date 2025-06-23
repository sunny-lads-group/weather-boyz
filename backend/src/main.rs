// src/main.rs
use axum::{
    Json, Router,
    extract::Query,
    routing::{get, post},
};
use serde::Deserialize;
use tower_http::cors::CorsLayer;

mod weather_data;
use weather_data::weatherxm::{WeatherResponse, get_weather_data_from_coords}; // Add WeatherResponse here

mod db;
use db::user_queries::create_user;

mod web;

#[derive(Deserialize)]
struct LocationQuery {
    lat: f64,
    lng: f64,
}

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
    let main_router = Router::new()
        .route("/getLocalWeather", get(get_local_weather))
        .route("/createUser", post(create_user));

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
    tracing::info!("  POST /signin - User authentication");
    tracing::info!("  GET  /getLocalWeather - Get weather data");
    tracing::info!("  POST /createUser - Create new user");
    tracing::info!("  GET  /tokenvalid/ - Protected route (requires auth)");

    if let Err(e) = axum::serve(listener, app).await {
        tracing::error!("Server error: {:?}", e);
        std::process::exit(1);
    }
}

async fn get_local_weather(
    Query(params): Query<LocationQuery>,
) -> Json<WeatherResponse> {
    let weather_data = get_weather_data_from_coords(params.lat, params.lng).await;
    Json(weather_data)
}