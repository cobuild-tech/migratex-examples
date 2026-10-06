import type { CurrentWeather, ForecastEntry } from '../../src/api/types';

/** Frozen "now" for forecast tests: Tue Oct 6 2026, 09:00 UTC. */
export const NOW = new Date('2026-10-06T09:00:00Z');

export const cities: Record<string, CurrentWeather> = {
  london: { name: 'London', sys: { country: 'GB' }, main: { temp: 14.27 }, weather: [{ description: 'clear sky', icon: '01d' }] },
  oslo: { name: 'Oslo', sys: { country: 'NO' }, main: { temp: -3.25 }, weather: [{ description: 'light snow', icon: '13d' }] },
  // Current weather works, the forecast request fails.
  atlantis: { name: 'Atlantis', sys: { country: 'GR' }, main: { temp: 20 }, weather: [{ description: 'mist', icon: '50d' }] },
};

/**
 * 40 entries, every 3 hours from NOW, like the real endpoint. That covers today (skipped),
 * Oct 7-10 in full, and Oct 11 up to 06:00 only, which has no 10:00-14:00 entry.
 * Noon entries are sunny; the rest are cloudy. Temperatures rise by 1° per day.
 */
export function forecastFor(cityTemp: number): { list: ForecastEntry[] } {
  const list = Array.from({ length: 40 }, (_, i) => {
    const date = new Date(NOW.getTime() + i * 3 * 3600 * 1000);
    const day = date.getUTCDate() - NOW.getUTCDate();
    const noon = date.getUTCHours() === 12;
    return {
      dt: date.getTime() / 1000,
      main: { temp_min: cityTemp + day - 2.4, temp_max: cityTemp + day + 2.6 },
      weather: [noon ? { description: 'clear sky', icon: '01d' } : { description: 'scattered clouds', icon: '03d' }],
    };
  });
  return { list };
}
