import { http, HttpResponse } from 'msw';
import { API_HOST, LEGACY_USER_SEARCH_URL } from '../../src/api/github';
import { events, KNOWN_USER, repos, user } from './data';

export const handlers = [
  http.get(`${LEGACY_USER_SEARCH_URL}:name`, ({ params }) =>
    HttpResponse.json({ users: params.name === KNOWN_USER ? [user] : [] }),
  ),
  http.get(`${API_HOST}/users/:name/events`, ({ params }) =>
    params.name === KNOWN_USER ? HttpResponse.json(events) : new HttpResponse(null, { status: 404 }),
  ),
  http.get(`${API_HOST}/users/:name/repos`, ({ params }) =>
    params.name === KNOWN_USER ? HttpResponse.json(repos) : new HttpResponse(null, { status: 404 }),
  ),
];
