import { formatCurrentTemp, formatForecastTemp, iconUrl } from '../../src/lib/format';

test('current temperature is floored to one decimal', () => {
  expect(formatCurrentTemp(14.27)).toBe('14.2°C');
  expect(formatCurrentTemp(14.29)).toBe('14.2°C');
  expect(formatCurrentTemp(-3.25)).toBe('-3.3°C');
  expect(formatCurrentTemp(20)).toBe('20.0°C');
});

test('forecast temperatures are rounded', () => {
  expect(formatForecastTemp(17.5)).toBe('18°C');
  expect(formatForecastTemp(-0.4)).toBe('0°C');
});

test('icon URL', () => {
  expect(iconUrl('10n')).toBe('https://openweathermap.org/img/wn/10n@2x.png');
});
