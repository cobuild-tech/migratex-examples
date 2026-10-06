/// <reference types="vitest" />
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

// Forecast days are grouped by local date, so pin the zone for predictable tests.
process.env.TZ = 'UTC';

export default defineConfig({
  plugins: [react()],
  test: {
    environment: 'jsdom',
    globals: true,
    setupFiles: ['./tests/setup.ts'],
    env: { VITE_OPENWEATHER_API_KEY: 'test-key' },
  },
});
