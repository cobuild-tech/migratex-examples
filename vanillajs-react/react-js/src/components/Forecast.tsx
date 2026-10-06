import { useMemo } from 'react';
import type { ForecastResponse } from '../api/types';
import { groupDailyForecast } from '../lib/forecast';
import { ForecastCard } from './ForecastCard';

export function Forecast({ data, onHide }: { data: ForecastResponse; onHide: () => void }) {
  const days = useMemo(() => groupDailyForecast(data.list), [data]);
  return (
    <section className="weather__section js-weather-forecast" id="weather-forecast">
      <h2 className="weather__section-title">5-Day Forecast</h2>
      <button className="weather__button js-weather-forecast-button-hide" id="forecast-button-hide" onClick={onHide}>
        Hide 5-Day Forecast
      </button>
      <div className="weather__forecast-list js-weather-forecast-list">
        {days.map((forecast) => (
          <ForecastCard key={forecast.day} forecast={forecast} />
        ))}
      </div>
    </section>
  );
}
