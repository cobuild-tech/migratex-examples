import { setupServer } from 'msw/node';
import { handlers } from './handlers';

export const server = setupServer(...handlers);

/** Every request the app made during the current test. */
export const requests: Request[] = [];
server.events.on('request:start', ({ request }) => {
  requests.push(request.clone());
});
