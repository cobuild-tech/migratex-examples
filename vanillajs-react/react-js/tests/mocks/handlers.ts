import { http, HttpResponse } from 'msw';
import { BASE_URL, FORECAST_BASE_URL } from '../../src/api/weather';
import { cities, forecastFor } from './data';

const NOT_FOUND = { cod: '404', message: 'city not found' };

const city = (request: Request) => cities[new URL(request.url).searchParams.get('q')!.toLowerCase()];

export const handlers = [
  http.get(BASE_URL, ({ request }) => {
    const weather = city(request);
    return weather ? HttpResponse.json(weather) : HttpResponse.json(NOT_FOUND, { status: 404 });
  }),
  http.get(FORECAST_BASE_URL, ({ request }) => {
    const weather = city(request);
    if (!weather) return HttpResponse.json(NOT_FOUND, { status: 404 });
    if (weather.name === 'Atlantis') return new HttpResponse(null, { status: 500 });
    return HttpResponse.json(forecastFor(weather.main.temp));
  }),
];
