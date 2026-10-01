import '@testing-library/jest-dom/vitest';
import { cleanup } from '@testing-library/react';
import { afterAll, afterEach, beforeAll, beforeEach } from 'vitest';
import { installMatchMedia, resetMedia } from './helpers/media';
import { requests, server } from './mocks/server';

beforeAll(() => server.listen({ onUnhandledRequest: 'error' }));
beforeEach(() => installMatchMedia());
afterEach(() => {
  cleanup();
  server.resetHandlers();
  requests.length = 0;
  resetMedia();
  document.body.className = '';
  vi.restoreAllMocks();
  vi.useRealTimers();
});
afterAll(() => server.close());
