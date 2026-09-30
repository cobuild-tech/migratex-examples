/// <reference types="vitest" />
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  test: {
    environment: 'jsdom',
    globals: true,
    setupFiles: ['./tests/polyfills.ts', './tests/setup.ts'],
    env: { VITE_API_HOST: 'http://api.test' },
  },
});
