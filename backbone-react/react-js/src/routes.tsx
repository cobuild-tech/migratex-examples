import { Navigate, Outlet, type RouteObject } from 'react-router-dom';
import { LoadingOverlay } from './components/LoadingOverlay';
import { HomePage } from './pages/HomePage';
import { ActivityPage, RepoCategoryPage, RepoPage } from './pages/PhonePages';

function Shell() {
  return (
    <>
      <Outlet />
      <LoadingOverlay />
    </>
  );
}

export const routes: RouteObject[] = [
  {
    element: <Shell />,
    children: [
      { path: '/', element: <HomePage /> },
      { path: '/activity', element: <ActivityPage /> },
      { path: '/repos', element: <RepoCategoryPage /> },
      { path: '/repos/:filter', element: <RepoPage /> },
      { path: '*', element: <Navigate to="/" replace state={{ transition: 'none' }} /> },
    ],
  },
];

export const routerFuture = {
  v7_relativeSplatPath: true,
  v7_fetcherPersist: true,
  v7_normalizeFormMethod: true,
  v7_partialHydration: true,
  v7_skipActionErrorRevalidation: true,
} as const;
