import type { ForecastEntry } from '../api/types';

export interface DailyForecast {
  day: string;
  tempMax: number;
  tempMin: number;
  icon: string;
  description: string;
}

const DAY_FORMAT: Intl.DateTimeFormatOptions = { weekday: 'short', month: 'short', day: 'numeric' };

const dayLabel = (date: Date) => date.toLocaleDateString('en-US', DAY_FORMAT);

/**
 * Groups 3-hourly entries into one card per local day, skipping today. Each day keeps its
 * min/max and the icon of its first 10:00-14:00 entry. Days with no midday entry (usually the
 * last, partial one) use the entry closest to noon instead of an empty icon.
 */
export function groupDailyForecast(list: ForecastEntry[], now = new Date()): DailyForecast[] {
  const today = dayLabel(now);
  const days = new Map<string, ForecastEntry[]>();

  for (const entry of list) {
    const day = dayLabel(new Date(entry.dt * 1000));
    if (day === today) continue;
    days.set(day, [...(days.get(day) ?? []), entry]);
  }

  return Array.from(days, ([day, entries]) => {
    const hour = (entry: ForecastEntry) => new Date(entry.dt * 1000).getHours();
    const midday =
      entries.find((entry) => hour(entry) >= 10 && hour(entry) <= 14) ??
      entries.reduce((best, entry) => (Math.abs(hour(entry) - 12) < Math.abs(hour(best) - 12) ? entry : best));

    return {
      day,
      tempMax: Math.max(...entries.map((entry) => entry.main.temp_max)),
      tempMin: Math.min(...entries.map((entry) => entry.main.temp_min)),
      icon: midday.weather[0].icon,
      description: midday.weather[0].description,
    };
  });
}
