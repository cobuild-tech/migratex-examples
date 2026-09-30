# Migration: Ember → React

The [`react-js/`](./react-js) app is a complete rewrite of [`ember-js/`](./ember-js), Inkwell, a
social blogging app backed by the public RealWorld API. It keeps the same routes, markup, CSS classes
and `data-test-*` selectors as the original, and every Ember acceptance test is ported. This document
describes how that migration was approached.

## Strategy

The app is small (18 components, 8 routes, 4 models), so a **full rewrite** was chosen over an
incremental migration. The whole Ember codebase (routes, templates, components, the session service,
Ember Data models/adapters/serializers, Mirage config and tests) was read before any React code was
written.

## Architectural mapping

| Ember concept | React equivalent | Notes |
|---|---|---|
| `router.js` + route classes | React Router 6 data router (`createBrowserRouter`) | Same URLs; the data router is needed for `useBlocker` |
| `LoggedInRoute.beforeModel` | `<RequireAuth>` layout route | Redirects to `/login` |
| Route `willTransition` unsaved-changes prompt | `useBlocker` + `window.confirm` in `ArticleForm` | |
| Index controller query params | `useSearchParams` | `feed`, `tag`, `page` |
| `session` service | `SessionContext` + `useSession()` | Same `localStorage` key (`ember-webapp.token`), so existing logins carry over |
| Ember Data models, adapters, serializers | Typed `fetch` client (`src/api`) + TanStack Query hooks | Slug/username are used directly as ids; no normalization layer |
| Ember Data identity map | `src/lib/cacheUpdates.ts` | Patches every cached copy after favorite/follow, as the store did |
| `ember-concurrency` restartable tasks | Query keys | Changing the key refetches, and stale responses are ignored |
| `model.errors` / `isValid` | `ApiError` with flattened `"attribute message"` strings | Same error-list markup |
| `hasDirtyAttributes` / `rollbackAttributes` | Local form state compared with initial values | Unmounting discards edits for free |
| Glimmer components + `.hbs` templates | Function components + JSX | 1:1 per component |
| `format-date` helper | `formatDate()` | Same `en-US` long format |
| Mirage | MSW + in-memory db | Factories and validators ported |
| QUnit acceptance tests | Vitest + React Testing Library | Same scenarios and selectors |

## Process followed

1. **Read the whole Ember codebase first**, including the Mirage mocks and acceptance tests, which define the expected behaviour.
2. **Scaffolded the React project** (Vite + React + TypeScript) with the RealWorld theme links from the original `index.html`.
3. **Built the foundations**: the API client, session, query hooks and cache sync.
4. **Migrated screen by screen**: shell and nav, home feed, auth, article and comments, editor, profile, settings.
5. **Ported every test**, including the regression tests for bugs in the Ember version (token carried after login, logged-out `?feed=your`, feed pagination, markdown sanitizing, tag whitespace, logout, author-only editor).
6. **Reviewed and hardened**: fixed cache resets on login/logout, transient-failure handling, URL-encoding of usernames and slugs, error reporting, double-submits and slug-change invalidation, each with a regression test.
7. **Verified in a real browser** against the live API (feeds, tags, article page, auth redirects, 404).

## Result

- Same routes, markup, CSS classes and test selectors
- Same session storage key, so users stay logged in across the switch
- 64/64 tests passing, clean TypeScript build, no console warnings

## Projects

| Folder | Stack |
|---|---|
| [`ember-js/`](./ember-js) | Ember 3.24 Octane + Ember Data + Mirage |
| [`react-js/`](./react-js) | React 18 + Vite + TypeScript + React Router 6 + TanStack Query |

Each folder is a standalone project with its own `package.json`, README and tests.
