// src/main.rs
use axum::{Json, Router, extract::Query, routing::get};
use serde::Deserialize;
use tower_http::cors::CorsLayer;

mod db;
mod weather_data;
use weather_data::weatherxm::{WeatherResponse, get_weather_data_from_coords}; // Add WeatherResponse here

#[derive(Deserialize)]
struct LocationQuery {
    lat: f64,
    lng: f64,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Test the database connection
    let pool = db::pool::get_pool().await.unwrap();

    let cors = CorsLayer::permissive();
    let app = Router::new()
        .route("/getLocalWeather", get(get_local_weather))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn get_local_weather(
    Query(params): Query<LocationQuery>,
) -> Json<WeatherResponse> {
    let weather_data = get_weather_data_from_coords(params.lat, params.lng).await;
    Json(weather_data)
}