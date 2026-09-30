import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { RouterProvider, type createBrowserRouter } from 'react-router-dom';
import { ApiError } from './api/client';
import { SessionProvider } from './session/SessionContext';

/** Retry transient failures (network errors, 5xx) a couple of times; 4xx answers won't change. */
export function shouldRetry(failureCount: number, error: unknown) {
  if (error instanceof ApiError && error.status < 500) return false;
  return failureCount < 2;
}

export function createQueryClient({ retry = true }: { retry?: boolean } = {}) {
  return new QueryClient({
    defaultOptions: { queries: { retry: retry ? shouldRetry : false, refetchOnWindowFocus: false } },
  });
}

export function App({
  router,
  queryClient,
}: {
  router: ReturnType<typeof createBrowserRouter>;
  queryClient: QueryClient;
}) {
  return (
    <QueryClientProvider client={queryClient}>
      <SessionProvider>
        <RouterProvider router={router} future={{ v7_startTransition: true }} />
      </SessionProvider>
    </QueryClientProvider>
  );
}
