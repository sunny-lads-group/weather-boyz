import React, { useState } from 'react';
import * as h3 from 'h3-js';

interface WeatherData {
  temperature: number;
  humidity: number;
  wind_speed: number;
  precipitation: number;
  feels_like: number;
}
interface Cell {
  index: string;
  device_count: number;
  active_device_count: number;
  avg_data_quality: number;
  center: {
    lat: number;
    lon: number;
  };
}

const LocationInput = () => {
  const [latitude, setLatitude] = useState<string>('');
  const [longitude, setLongitude] = useState<string>('');
  const [latError, setLatError] = useState<string>('');
  const [longError, setLongError] = useState<string>('');
  const [loading, setLoading] = useState<boolean>(false);
  const [weatherData, setWeatherData] = useState<WeatherData | null>(null);

  const findNearestDevice = async (lat: string, long: string) => {
    const userLat = parseFloat(lat);
    const userLong = parseFloat(long);
    const userHex = h3.latLngToCell(userLat, userLong, 7);
    console.log(`Searching for devices near hex: ${userHex}`);

    try {
      const response = await fetch(`https://api.weatherxm.com/api/v1/cells`);
      if (!response.ok) {
        throw new Error('Failed to fetch hex cells');
      }
      const allCellsWithDevices: Cell[] = await response.json();

      let nearestCell = null;
      let shortestDistance = Infinity;

      for (const cell of allCellsWithDevices) {
        let distance = Infinity;
        try {
          distance = h3.gridDistance(userHex, cell.index);
        } catch (error) {
          continue;
        }
        if (distance < shortestDistance) {
          shortestDistance = distance;
          nearestCell = cell;
        }
      }

      if (!nearestCell) {
        throw new Error('No cells with devices found');
      }

      const deviceResponse = await fetch(
        `https://api.weatherxm.com/api/v1/cells/${nearestCell.index}/devices`
      );

      if (!deviceResponse.ok) {
        throw new Error('Failed to fetch devices for the nearest cell');
      }

      const deviceData = await deviceResponse.json();

      if (!deviceData || deviceData.length === 0) {
        throw new Error('No devices found in the nearest cell');
      }

      const weatherData = {
        temperature: deviceData[0].current_weather.temperature,
        humidity: deviceData[0].current_weather.humidity,
        wind_speed: deviceData[0].current_weather.wind_speed,
        precipitation: deviceData[0].current_weather.precipitation,
        feels_like: deviceData[0].current_weather.feels_like,
      };

      setWeatherData(weatherData);
    } catch (error) {
      console.error('Error:', error);
    } finally {
      setLoading(false);
    }
  };

  const fetchWeatherData = async (lat: string, long: string) => {
    try {
      await findNearestDevice(lat, long);
    } catch (error) {
      console.error('Error fetching weather data:', error);
      alert('Failed to fetch weather data');
      setLoading(false);
    }
  };

  const validateLatitude = (value: string) => {
    const lat = parseFloat(value);
    if (isNaN(lat)) {
      setLatError('Please enter a valid number');
      return false;
    } else if (lat < -90 || lat > 90) {
      setLatError('Latitude must be between -90 and 90');
      return false;
    }
    setLatError('');
    return true;
  };

  const validateLongitude = (value: string) => {
    const lng = parseFloat(value);
    if (isNaN(lng)) {
      setLongError('Please enter a valid number');
      return false;
    } else if (lng < -180 || lng > 180) {
      setLongError('Longitude must be between -180 and 180');
      return false;
    }
    setLongError('');
    return true;
  };

  const handleLatChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value;
    setLatitude(value);
    validateLatitude(value);
  };

  const handleLongChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value;
    setLongitude(value);
    validateLongitude(value);
  };

  const handleSubmitCoordinates = () => {
    if (validateLatitude(latitude) && validateLongitude(longitude)) {
      setLoading(true);
      fetchWeatherData(latitude, longitude);
    }
  };

  const getLocation = () => {
    if (!navigator.geolocation) {
      alert('Geolocation is not supported by your browser');
      return;
    }
    setLoading(true);
    navigator.geolocation.getCurrentPosition(
      (position) => {
        const lat = position.coords.latitude.toString();
        const long = position.coords.longitude.toString();
        setLatitude(lat);
        setLongitude(long);
        fetchWeatherData(lat, long);
      },
      (error) => {
        alert(`Error getting location: ${error.message}`);
        setLoading(false);
      }
    );
  };

  if (weatherData) {
    return (
      <div className="bg-white rounded-lg shadow-lg p-6">
        <h2 className="text-xl font-bold mb-4">Current Weather Conditions</h2>
        <div className="grid grid-cols-2 gap-4">
          <div className="p-3 bg-gray-50 rounded">
            <p className="text-gray-600">Temperature</p>
            <p className="text-xl font-semibold">{weatherData.temperature}°C</p>
          </div>
          <div className="p-3 bg-gray-50 rounded">
            <p className="text-gray-600">Feels Like</p>
            <p className="text-xl font-semibold">{weatherData.feels_like}°C</p>
          </div>
          <div className="p-3 bg-gray-50 rounded">
            <p className="text-gray-600">Humidity</p>
            <p className="text-xl font-semibold">{weatherData.humidity}%</p>
          </div>
          <div className="p-3 bg-gray-50 rounded">
            <p className="text-gray-600">Wind Speed</p>
            <p className="text-xl font-semibold">
              {weatherData.wind_speed} m/s
            </p>
          </div>
          <div className="p-3 bg-gray-50 rounded">
            <p className="text-gray-600">Precipitation</p>
            <p className="text-xl font-semibold">
              {weatherData.precipitation} mm
            </p>
          </div>
        </div>
        <div className="mt-4">
          <p className="text-sm text-gray-600">
            Location: {latitude}, {longitude}
          </p>
        </div>
        <button
          onClick={() => setWeatherData(null)}
          className="mt-4 px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600"
        >
          Reset
        </button>
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-5 p-4">
      <button
        onClick={getLocation}
        disabled={loading}
        className={`px-4 py-2 text-white rounded-md ${
          loading
            ? 'bg-gray-400 cursor-not-allowed'
            : 'bg-green-500 hover:bg-green-600 cursor-pointer'
        }`}
      >
        {loading ? 'Getting location...' : 'Get Current Location'}
      </button>
      <div>
        <h3 className="mb-4">Or enter coordinates manually:</h3>
        <div className="mb-3">
          <label htmlFor="latitude" className="mr-2">
            Latitude:
          </label>
          <input
            id="latitude"
            type="text"
            value={latitude}
            onChange={handleLatChange}
            placeholder="-90 to 90"
            className="px-2 py-1 border rounded"
          />
          {latError && <p className="text-red-500 mt-1">{latError}</p>}
        </div>
        <div className="mb-4">
          <label htmlFor="longitude" className="mr-2">
            Longitude:
          </label>
          <input
            id="longitude"
            type="text"
            value={longitude}
            onChange={handleLongChange}
            placeholder="-180 to 180"
            className="px-2 py-1 border rounded"
          />
          {longError && <p className="text-red-500 mt-1">{longError}</p>}
        </div>
        <button
          onClick={handleSubmitCoordinates}
          disabled={loading || !!latError || !!longError}
          className={`px-4 py-2 text-white rounded-md ${
            loading || !!latError || !!longError
              ? 'bg-gray-400 cursor-not-allowed'
              : 'bg-blue-500 hover:bg-blue-600 cursor-pointer'
          }`}
        >
          Submit Coordinates
        </button>
      </div>
    </div>
  );
};

export default LocationInput;
