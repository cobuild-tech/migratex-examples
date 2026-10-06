import type { ForecastEntry } from '../../src/api/types';
import { groupDailyForecast } from '../../src/lib/forecast';

const entry = (iso: string, min: number, max: number, icon = '03d'): ForecastEntry => ({
  dt: new Date(iso).getTime() / 1000,
  main: { temp_min: min, temp_max: max },
  weather: [{ description: icon, icon }],
});

const now = new Date('2026-10-06T21:00:00Z');

test('skips today and keeps each day in order with its min and max', () => {
  const days = groupDailyForecast(
    [
      entry('2026-10-06T21:00:00Z', 0, 50),
      entry('2026-10-07T00:00:00Z', 4, 6),
      entry('2026-10-07T12:00:00Z', 8, 15),
      entry('2026-10-07T21:00:00Z', 2, 5),
      entry('2026-10-08T03:00:00Z', -1, 1),
    ],
    now,
  );
  expect(days.map(({ day, tempMin, tempMax }) => [day, tempMin, tempMax])).toEqual([
    ['Wed, Oct 7', 2, 15],
    ['Thu, Oct 8', -1, 1],
  ]);
});

test('uses the first 10:00-14:00 entry for the icon', () => {
  const [day] = groupDailyForecast(
    [
      entry('2026-10-07T09:00:00Z', 0, 0, '02d'),
      entry('2026-10-07T12:00:00Z', 0, 0, '01d'),
      entry('2026-10-07T15:00:00Z', 0, 0, '10d'),
    ],
    now,
  );
  expect(day.icon).toBe('01d');
});

test('falls back to the entry closest to noon', () => {
  const [day] = groupDailyForecast(
    [entry('2026-10-07T00:00:00Z', 0, 0, '01n'), entry('2026-10-07T06:00:00Z', 0, 0, '02d'), entry('2026-10-07T21:00:00Z', 0, 0, '03n')],
    now,
  );
  expect(day.icon).toBe('02d');
});

test('an empty list gives no days', () => {
  expect(groupDailyForecast([], now)).toEqual([]);
});
