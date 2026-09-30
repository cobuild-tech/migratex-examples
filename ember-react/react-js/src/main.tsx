import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { createBrowserRouter } from 'react-router-dom';
import { App, createQueryClient } from './App';
import { routerFuture, routes } from './routes';

const router = createBrowserRouter(routes, { future: routerFuture });

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App router={router} queryClient={createQueryClient()} />
  </StrictMode>,
);
