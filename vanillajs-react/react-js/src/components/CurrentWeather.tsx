import type { CurrentWeather as Weather } from '../api/types';
import { formatCurrentTemp, iconUrl } from '../lib/format';

interface Props {
  weather: Weather;
  forecastOpen: boolean;
  forecastLoading: boolean;
  onShowForecast: () => void;
}

export function CurrentWeather({ weather, forecastOpen, forecastLoading, onShowForecast }: Props) {
  const { description, icon } = weather.weather[0];
  return (
    <section className="weather__section js-weather-result">
      <div className="weather__result">
        <h2 className="weather__section-title js-weather-city-name">{`${weather.name}, ${weather.sys.country}`}</h2>
        <p className="weather__temperature js-weather-temperature">{formatCurrentTemp(weather.main.temp)}</p>
        <img src={iconUrl(icon)} alt={description} className="weather__icon js-weather-icon" />
        <p className="weather__description js-weather-description">{description}</p>
      </div>
      <button
        className={`weather__button js-weather-forecast-button${forecastOpen ? ' is-hidden' : ''}`}
        id="forecast-button"
        disabled={forecastLoading}
        onClick={onShowForecast}
      >
        Show 5-Day Forecast
      </button>
    </section>
  );
}
