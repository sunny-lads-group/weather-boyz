use h3o::{LatLng, Resolution};
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct WeatherXMResponse {
    current_weather: CurrentWeather,
}

#[derive(Deserialize)]
struct CurrentWeather {
    temperature: f64,
    humidity: i32,
    wind_speed: f64,
    precipitation: f64,
    feels_like: f64,
}

#[derive(Serialize, Deserialize)]
pub struct WeatherResponse {
    pub temperature: f64,
    pub humidity: i32,
    pub wind_speed: f64,
    pub precipitation: f64,
    pub feels_like: f64,
}

pub async fn get_weather_data_from_coords(lat: f64, lng: f64) -> WeatherResponse {
    let location: LatLng = LatLng::new(lat, lng).expect("Invalid coordinates");
    let cell_id: h3o::CellIndex = location.to_cell(Resolution::Seven);
    let cell_id_str: String = cell_id.to_string();

    let url: String = format!("https://api.weatherxm.com/api/v1/cells/{}/devices", cell_id_str);
    
    let client: reqwest::Client = reqwest::Client::new();
    let response: reqwest::Response = client.get(&url)
        .send()
        .await
        .expect("Failed to get weather data");


    let weather_data: Vec<WeatherXMResponse> = response.json()
        .await
        .expect("Failed to parse weather data");

    let current_weather = &weather_data[0].current_weather;

    WeatherResponse {
        temperature: current_weather.temperature,
        humidity: current_weather.humidity,
        wind_speed: current_weather.wind_speed,
        precipitation: current_weather.precipitation,
        feels_like: current_weather.feels_like,
    }
}