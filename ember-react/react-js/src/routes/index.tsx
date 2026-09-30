import { Outlet, type RouteObject } from 'react-router-dom';
import { Footer } from '../components/Footer';
import { Nav } from '../components/Nav';
import { ArticlePage } from '../pages/ArticlePage';
import { LoginPage, RegisterPage, SettingsPage } from '../pages/AuthPages';
import { EditorEditPage, EditorLayout, EditorNewPage } from '../pages/EditorPages';
import { HomePage } from '../pages/HomePage';
import { NotFoundPage } from '../pages/NotFoundPage';
import { ProfileArticlesPage, ProfileFavoritesPage, ProfileLayout } from '../pages/ProfilePages';
import { RequireAuth } from './RequireAuth';

function ApplicationLayout() {
  return (
    <>
      <Nav />
      <Outlet />
      <Footer />
    </>
  );
}

export const routes: RouteObject[] = [
  {
    element: <ApplicationLayout />,
    children: [
      { index: true, element: <HomePage /> },
      { path: 'login', element: <LoginPage /> },
      { path: 'register', element: <RegisterPage /> },
      { path: 'articles/:id', element: <ArticlePage /> },
      {
        path: 'profile/:id',
        element: <ProfileLayout />,
        children: [
          { index: true, element: <ProfileArticlesPage /> },
          { path: 'favorites', element: <ProfileFavoritesPage /> },
        ],
      },
      {
        element: <RequireAuth />,
        children: [
          { path: 'settings', element: <SettingsPage /> },
          {
            path: 'editor',
            element: <EditorLayout />,
            children: [
              { index: true, element: <EditorNewPage /> },
              { path: ':id', element: <EditorEditPage /> },
            ],
          },
        ],
      },
      { path: '*', element: <NotFoundPage /> },
    ],
  },
];

// Opt into React Router v7 behaviors now to keep the upgrade path clean.
export const routerFuture = {
  v7_relativeSplatPath: true,
  v7_fetcherPersist: true,
  v7_normalizeFormMethod: true,
  v7_partialHydration: true,
  v7_skipActionErrorRevalidation: true,
};
