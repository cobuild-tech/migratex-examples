# Backbone Responsive Example (React)

React port of [`../backbone-js`](../backbone-js): Vite + TypeScript + React Router 6 + TanStack Query.

```bash
npm install
npm run dev        # http://localhost:5173, talks to the live GitHub API
npm run dev:mock   # same, with canned GitHub data (search for "octocat")
npm test           # Vitest + Testing Library + MSW
npm run build      # typecheck + production build
```

At 600px and wider, everything renders as panels on the home page. Below that, activity, repo
categories and repos are separate pages (`/activity`, `/repos`, `/repos/:filter`) with a Back header.

## Backbone → React map
| Backbone | React |
|---|---|
| `controller.js` (`isPhone`, `goTo*`, `changePage`) | `src/hooks/useIsPhone.ts`, `src/routes.tsx`, `src/pages/*` |
| `globals.js` | `src/state/AppState.tsx` |
| `models/*`, `collections/*` | `src/api/github.ts` + `src/hooks/queries.ts` |
| `views/homeView.js` + `templates/pages/home.html` | `src/pages/HomePage.tsx` |
| `views/activityView.js`, `repoCategoryView.js`, `repoView.js` | `src/components/ActivityPanel.tsx`, `RepoCategories.tsx`, `RepoTable.tsx`, wrapped by `src/pages/PhonePages.tsx` on phones |
| `templates/partials/header.html`, `footer.html` | `src/components/Header.tsx`, `Footer.tsx` |
| `events.js` (page stack, `$.ajaxSetup` loader) | Unmounting; `src/components/LoadingOverlay.tsx` |
| jQuery Mobile CSS, `app.css`, `app-phone.css` | `src/styles/app.css` |
