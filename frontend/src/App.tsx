import { useState, useEffect } from 'react'
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom'
import './index.css'
import WalletConnect from './components/WalletConnect'
import Navbar from './components/Navbar'
import Policies from './pages/Policies'

interface WeatherData {
  temperature: number;
  humidity: number;
  wind_speed: number;
  precipitation: number;
  feels_like: number;
}

function App() {
  const [connectedAddress, setConnectedAddress] = useState<string>('')
  const [weatherData, setWeatherData] = useState<WeatherData | null>(null)

  const getLocation = async () => {
    if ("geolocation" in navigator) {
      return new Promise((resolve, reject) => {
        navigator.geolocation.getCurrentPosition(
          (position) => resolve({
            lat: position.coords.latitude,
            lng: position.coords.longitude
          }),
          (error) => reject(error)
        );
      });
    } else {
      throw new Error("Geolocation is not supported");
    }
  };

  const fetchWeatherData = async (coords: { lat: number, lng: number }) => {
    try {
      const response = await fetch(`http://localhost:3000/getLocalWeather?lat=${coords.lat}&lng=${coords.lng}`);
      const data = await response.json();
      setWeatherData(data);
    } catch (error) {
      console.error("Error fetching weather data:", error);
    }
  };

  const handleWalletConnect = async (address: string) => {
    setConnectedAddress(address);
    try {
      const coords = await getLocation() as { lat: number, lng: number };
      await fetchWeatherData(coords);
    } catch (error) {
      console.error("Error getting location:", error);
    }
  };

  return (
    <Router>

      <div >
        <Navbar />
        <main >
          <Routes>
            <Route path="/" element={
              <>
                <h1>Weather Insurance</h1>
                <WalletConnect onConnect={handleWalletConnect} />
                {connectedAddress && (
                  <div >
                    <p>Connected as: {connectedAddress}</p>
                    {weatherData && (
                      <div >
                        <h2>Current Weather</h2>
                        <p>Temperature: {weatherData.temperature}°C</p>
                        <p>Humidity: {weatherData.humidity}%</p>
                        <p>Wind Speed: {weatherData.wind_speed} m/s</p>
                        <p>Precipitation: {weatherData.precipitation} mm</p>
                        <p>Feels Like: {weatherData.feels_like}°C</p>
                      </div>
                    )}
                  </div>
                )}
              </>
            } />
            <Route path="/policies" element={<Policies />} />
          </Routes>
        </main>
      </div>
    </Router>
  )
}

export default App