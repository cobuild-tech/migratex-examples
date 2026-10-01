import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { createBrowserRouter } from 'react-router-dom';
import { App, createQueryClient } from './App';
import { routerFuture, routes } from './routes';

async function start() {
  // `npm run dev:mock` serves canned GitHub data, since the legacy user search is gone.
  if (import.meta.env.DEV && import.meta.env.VITE_MOCK) {
    const { worker } = await import('../tests/mocks/browser');
    await worker.start({ onUnhandledRequest: 'bypass' });
  }
  const router = createBrowserRouter(routes, { future: routerFuture });
  createRoot(document.getElementById('root')!).render(
    <StrictMode>
      <App router={router} queryClient={createQueryClient()} />
    </StrictMode>,
  );
}

start();
