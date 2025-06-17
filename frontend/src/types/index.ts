export interface WeatherData {
  temperature: number;
  humidity: number;
  wind_speed: number;
  precipitation: number;
  feels_like: number;
}

export interface Coordinates {
  lat: number;
  lng: number;
}