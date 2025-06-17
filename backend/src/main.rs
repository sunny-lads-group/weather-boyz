// src/main.rs
use axum::{
    Router,
    routing::get,
    extract::Query,
    Json,
};
use tower_http::cors::CorsLayer;
use serde::{Deserialize};

mod weather_data;
use weather_data::weatherxm::{get_weather_data_from_coords, WeatherResponse}; // Add WeatherResponse here

#[derive(Deserialize)]
struct LocationQuery {
    lat: f64,
    lng: f64,
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::permissive();
    let app = Router::new()
        .route("/getLocalWeather", get(get_local_weather))
        .layer(cors);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn get_local_weather(
    Query(params): Query<LocationQuery>,
) -> Json<WeatherResponse> {
    let weather_data = get_weather_data_from_coords(params.lat, params.lng).await;
    Json(weather_data)
}