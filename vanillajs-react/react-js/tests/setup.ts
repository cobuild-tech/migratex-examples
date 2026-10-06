import '@testing-library/jest-dom/vitest';
import { cleanup } from '@testing-library/react';
import { afterAll, afterEach, beforeAll } from 'vitest';
import { requests, server } from './mocks/server';

beforeAll(() => server.listen({ onUnhandledRequest: 'error' }));
afterEach(() => {
  cleanup();
  server.resetHandlers();
  requests.length = 0;
  localStorage.clear();
  document.documentElement.setAttribute('data-theme', 'light');
  vi.unstubAllEnvs();
  vi.useRealTimers();
});
afterAll(() => server.close());
