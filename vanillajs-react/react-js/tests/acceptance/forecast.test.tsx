import { visit } from '../helpers/app';
import { requests } from '../mocks/server';

describe('5-day forecast', () => {
  test('shows one card per day after today', async () => {
    const app = visit();
    await app.searchFor('London');
    expect(app.$('.js-weather-forecast')).toBeNull();

    await app.showForecast();

    expect(app.$('.js-weather-forecast h2')).toHaveTextContent('5-Day Forecast');
    expect(app.$('#forecast-button')).toHaveClass('is-hidden');
    expect(app.forecastDays()).toEqual(['Wed, Oct 7', 'Thu, Oct 8', 'Fri, Oct 9', 'Sat, Oct 10', 'Sun, Oct 11']);

    const [first] = app.$$('.weather__forecast-item');
    expect(first.querySelector('img')).toHaveAttribute('src', 'https://openweathermap.org/img/wn/01d@2x.png');
    expect(first.querySelector('img')).toHaveAttribute('alt', 'clear sky');
    expect(first).toHaveTextContent('Max: 18°C'); // 14.27 + 1 + 2.6 = 17.87
    expect(first).toHaveTextContent('Min: 13°C'); // 14.27 + 1 - 2.4 = 12.87
  });

  test('a day without a midday entry still gets an icon', async () => {
    const app = visit();
    await app.searchFor('London');
    await app.showForecast();

    const last = app.$$('.weather__forecast-item').slice(-1)[0];
    expect(last.querySelector('img')).toHaveAttribute('src', 'https://openweathermap.org/img/wn/03d@2x.png');
  });

  test('hide and show again', async () => {
    const app = visit();
    await app.searchFor('London');
    await app.showForecast();

    await app.user.click(app.$('#forecast-button-hide')!);
    expect(app.$('.js-weather-forecast')).toBeNull();
    expect(app.$('#forecast-button')).not.toHaveClass('is-hidden');

    await app.showForecast();
    expect(app.forecastDays()).toHaveLength(5);
  });

  test('is for the searched city, even if the input has changed since', async () => {
    const app = visit();
    await app.searchFor('London');
    await app.user.clear(app.$('.js-weather-input')!);
    await app.user.type(app.$('.js-weather-input')!, 'Oslo');
    await app.showForecast();

    const forecastRequest = requests.find((request) => request.url.includes('/forecast'))!;
    expect(new URL(forecastRequest.url).searchParams.get('q')).toBe('London');
  });

  test('a new search closes the previous forecast', async () => {
    const app = visit();
    await app.searchFor('London');
    await app.showForecast();
    await app.searchFor('Oslo');

    expect(app.$('.js-weather-forecast')).toBeNull();
    expect(app.$('#forecast-button')).not.toHaveClass('is-hidden');
  });

  test('a failed forecast shows the error and can be retried', async () => {
    const app = visit();
    await app.searchFor('Atlantis');
    await app.showForecast();

    expect(app.$('#error-container')).toBeInTheDocument();
    expect(app.$('.js-weather-result')).toBeInTheDocument();
    expect(app.$('.js-weather-forecast')).toBeNull();

    await app.showForecast();
    expect(requests.filter((request) => request.url.includes('/forecast'))).toHaveLength(2);
  });
});
