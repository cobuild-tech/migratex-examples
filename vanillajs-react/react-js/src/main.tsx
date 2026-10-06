import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { App, createQueryClient } from './App';

async function start() {
  // `npm run dev:mock` serves canned weather data, so no API key is needed.
  if (import.meta.env.DEV && import.meta.env.VITE_MOCK) {
    const { worker } = await import('../tests/mocks/browser');
    await worker.start({ onUnhandledRequest: 'bypass' });
  }
  createRoot(document.getElementById('root')!).render(
    <StrictMode>
      <App queryClient={createQueryClient()} />
    </StrictMode>,
  );
}

start();
