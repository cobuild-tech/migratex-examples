/** Current temperature: rounded down to one decimal, as the original did. */
export function formatCurrentTemp(temp: number) {
  return `${(Math.floor(temp * 10) / 10).toFixed(1)}°C`;
}

/** Forecast temperatures: rounded to whole degrees. */
export function formatForecastTemp(temp: number) {
  return `${Math.round(temp).toFixed(0)}°C`;
}

export function iconUrl(icon: string) {
  return `https://openweathermap.org/img/wn/${icon}@2x.png`;
}
