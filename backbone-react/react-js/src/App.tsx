import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { RouterProvider, type createBrowserRouter } from 'react-router-dom';
import { AppStateProvider } from './state/AppState';
import './styles/app.css';

type Router = ReturnType<typeof createBrowserRouter>;

export function createQueryClient() {
  // Backbone fetched once per click, with no retries or background refetching.
  return new QueryClient({
    defaultOptions: {
      queries: { retry: false, refetchOnWindowFocus: false, staleTime: Infinity },
      mutations: { retry: false },
    },
  });
}

export function App({ router, queryClient }: { router: Router; queryClient: QueryClient }) {
  return (
    <QueryClientProvider client={queryClient}>
      <AppStateProvider>
        <RouterProvider router={router} future={{ v7_startTransition: true }} />
      </AppStateProvider>
    </QueryClientProvider>
  );
}
