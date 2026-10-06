import type { CurrentWeather, ForecastResponse } from './types';

export const BASE_URL = 'https://api.openweathermap.org/data/2.5/weather';
export const FORECAST_BASE_URL = 'https://api.openweathermap.org/data/2.5/forecast';
const UNITS = 'metric';
const LANG = 'en';

// NOTE: a key in a frontend bundle is still public. A real app would call the API through a
// backend proxy. Vite inlines the key from .env.local at build time.
export class MissingApiKeyError extends Error {
  constructor() {
    super('Missing OpenWeatherMap API key. Set VITE_OPENWEATHER_API_KEY in .env.local.');
  }
}

function apiKey() {
  const key = import.meta.env.VITE_OPENWEATHER_API_KEY;
  if (key) return key;
  // `npm run dev:mock` answers from canned data, so any key will do.
  if (import.meta.env.VITE_MOCK) return 'mock';
  throw new MissingApiKeyError();
}

async function get<T>(baseUrl: string, cityName: string, notFound: string): Promise<T> {
  const params = new URLSearchParams({ q: cityName, appid: apiKey(), units: UNITS, lang: LANG });
  const response = await fetch(`${baseUrl}?${params}`);
  if (!response.ok) throw new Error(notFound);
  return response.json();
}

export function getWeatherByCity(cityName: string) {
  return get<CurrentWeather>(BASE_URL, cityName, `City not found: ${cityName}`);
}

export function getWeatherForecastByCity(cityName: string) {
  return get<ForecastResponse>(FORECAST_BASE_URL, cityName, `Forecast data not found for: ${cityName}`);
}
