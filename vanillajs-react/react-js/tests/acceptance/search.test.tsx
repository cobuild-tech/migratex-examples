import { http, HttpResponse } from 'msw';
import { BASE_URL } from '../../src/api/weather';
import { visit } from '../helpers/app';
import { cities } from '../mocks/data';
import { requests, server } from '../mocks/server';

describe('searching for a city', () => {
  test('shows the current weather', async () => {
    const app = visit();
    expect(app.$('.js-weather-result')).toBeNull();

    await app.searchFor('London');

    expect(app.$('.js-weather-city-name')).toHaveTextContent('London, GB');
    expect(app.$('.js-weather-temperature')).toHaveTextContent('14.2°C');
    expect(app.$('.js-weather-description')).toHaveTextContent('clear sky');
    expect(app.$('.js-weather-icon')).toHaveAttribute('src', 'https://openweathermap.org/img/wn/01d@2x.png');
    expect(app.$('.js-weather-icon')).toHaveAttribute('alt', 'clear sky');
    expect(app.$('#error-container')).toBeNull();
  });

  test('sends the same query as the original app', async () => {
    const app = visit();
    await app.searchFor('  New York ');

    const url = new URL(requests[0].url);
    expect(url.origin + url.pathname).toBe('https://api.openweathermap.org/data/2.5/weather');
    expect(Object.fromEntries(url.searchParams)).toEqual({ q: 'New York', appid: 'test-key', units: 'metric', lang: 'en' });
  });

  test('rounds the temperature down to one decimal, below zero too', async () => {
    const app = visit();
    await app.searchFor('Oslo');
    expect(app.$('.js-weather-temperature')).toHaveTextContent('-3.3°C');
  });

  test('pressing Enter searches', async () => {
    const app = visit();
    await app.searchFor('London', { enter: true });
    expect(app.$('.js-weather-city-name')).toHaveTextContent('London, GB');
  });

  test('an empty or blank input does nothing', async () => {
    const app = visit();
    await app.user.click(app.$('.js-weather-search-button')!);
    await app.user.type(app.$('.js-weather-input')!, '   {Enter}');
    expect(requests).toHaveLength(0);
  });

  test('an unknown city shows the error and hides the result', async () => {
    const app = visit();
    await app.searchFor('London');
    await app.searchFor('Nowhereville');

    expect(app.$('.js-weather-result')).toBeNull();
    expect(app.$('#error-container')).toHaveTextContent('Oops! Could not fetch weather data. Please try again later.');
    expect(app.$('.error-icon')).toHaveAttribute('alt', '');

    await app.searchFor('London');
    expect(app.$('#error-container')).toBeNull();
  });

  test('the search button shows a loading state', async () => {
    let respond!: () => void;
    const responded = new Promise<void>((resolve) => (respond = resolve));
    server.use(http.get(BASE_URL, async () => (await responded, HttpResponse.json(cities.london))));
    const app = visit();
    await app.user.type(app.$('.js-weather-input')!, 'London');
    await app.user.click(app.$('.js-weather-search-button')!);
    expect(app.$('.js-weather-search-button')).toHaveTextContent('Loading...');
    expect(app.$('.js-weather-search-button')).toBeDisabled();
    expect(app.$('.js-weather-result')).toBeNull();
    respond();
    await app.find('.js-weather-result');
  });

  test('searching the same city again fetches again', async () => {
    const app = visit();
    await app.searchFor('London');
    await app.searchFor('London');
    expect(requests).toHaveLength(2);
  });

  test('a missing API key is reported instead of sending a request', async () => {
    vi.stubEnv('VITE_OPENWEATHER_API_KEY', '');
    const app = visit();
    await app.searchFor('London');
    expect(requests).toHaveLength(0);
    expect(app.$('#error-container')).toHaveTextContent('Set VITE_OPENWEATHER_API_KEY in .env.local');
  });
});
