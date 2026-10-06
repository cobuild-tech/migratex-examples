import type { DailyForecast } from '../lib/forecast';
import { formatForecastTemp, iconUrl } from '../lib/format';

export function ForecastCard({ forecast }: { forecast: DailyForecast }) {
  return (
    <div className="weather__forecast-item">
      <h3>{forecast.day}</h3>
      <img src={iconUrl(forecast.icon)} alt={forecast.description} className="weather__forecast-icon" />
      <p>Max: {formatForecastTemp(forecast.tempMax)}</p>
      <p>Min: {formatForecastTemp(forecast.tempMin)}</p>
    </div>
  );
}
