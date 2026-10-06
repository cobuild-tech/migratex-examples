/** The fields of OpenWeatherMap's `/data/2.5/weather` response the app uses. */
export interface CurrentWeather {
  name: string;
  sys: { country: string };
  main: { temp: number };
  weather: WeatherCondition[];
}

export interface WeatherCondition {
  description: string;
  icon: string;
}

/** One 3-hourly entry of `/data/2.5/forecast`. */
export interface ForecastEntry {
  dt: number;
  main: { temp_min: number; temp_max: number };
  weather: WeatherCondition[];
}

export interface ForecastResponse {
  list: ForecastEntry[];
}
