# Migration: Backbone → React

The [`react-js/`](./react-js) app is a complete rewrite of [`backbone-js/`](./backbone-js), a responsive
GitHub user viewer. It looks up a user, then shows their recent activity and their repositories,
filtered by all, source or fork. On desktop every view is a panel on one page. On phones (below 600px)
each view is its own page with a slide transition and a Back button. The rewrite keeps the same
screens, text, element ids and API calls. This document describes how that migration was approached.

## Strategy

The app is small (4 views, 3 models, 2 collections, a controller and 9 Handlebars templates), so a
**full rewrite** was chosen over an incremental migration. The Backbone app had no tests, so its
behaviour was recorded from the code first and then written down as the React test suite.

Most of the original code was plumbing for jQuery Mobile: a page stack, a `page:destroy` event bus,
`pagecreate` re-triggers and separate touch and click event maps. React's rendering removes the need
for all of it, so the port is mostly the screens themselves.

## Architectural mapping

| Backbone concept | React equivalent | Notes |
|---|---|---|
| RequireJS `main.js` + `Globals` singleton | ES modules + `AppStateContext` | The singleton only existed to break a circular RequireJS dependency |
| `controller.js` instead of a router | React Router 6 (`/`, `/activity`, `/repos`, `/repos/:filter`) | Phone pages now have URLs and the browser Back button works |
| EnquireJS one-time phone check | `useIsPhone()`: `matchMedia` + `useSyncExternalStore` | Live: resizing switches layouts |
| `goTo*` passing `el` on desktop, `changePage` on phone | Desktop renders panels inside the home page; phone navigates to a page | Same components in both layouts |
| `UserModel`, `ActivityCollection`, `RepoCollection` | `fetch` functions + TanStack Query hooks | Each click is a new request, as each Backbone view built a new collection |
| `{{#if isPhone}}` header/footer in templates | `<PhonePage>` wrapper | |
| `page:destroy` bus, `pages` stack, `BaseView.close` | Unmounting | |
| `$.ajaxSetup` "Connecting To Github" loader | `<LoadingOverlay>` from `useIsFetching` / `useIsMutating` | Also disables the page, like `ui-disabled` |
| jQuery `slideDown` / `slideUp` | CSS `grid-template-rows` transition | "No User Found" alert still waits for the slide |
| JQM slide transitions (`reverse` for Back) | CSS keyframes, `reverse` passed in navigation state | |
| jQuery Mobile 1.3 theme CSS | Plain CSS recreation (`src/styles/app.css`) | No jQuery or jQuery Mobile |
| `_.where(repos, {fork})` | `filterRepos()` | |

## Process followed

1. **Read the whole Backbone app**, including the controller, the page-stack event handling and the templates, and listed every behaviour, including the odd ones: a failed lookup does nothing, the alert waits for the avatar to slide up, and pressing Get User Repositories again clears the repo table.
2. **Scaffolded the React project** (Vite + React + TypeScript), matching the stack of the other examples.
3. **Built the foundations**: the API functions, query hooks, shared state and the breakpoint hook.
4. **Migrated screen by screen**: home and user lookup, activity, repo categories, repo table, then the phone page wrappers and routes.
5. **Recreated the jQuery Mobile look** in plain CSS: bars, buttons, grids, tables, loader and transitions.
6. **Wrote the tests** against MSW mocks of the three GitHub endpoints, covering the desktop, phone and resize flows.
7. **Verified in a real browser** at 1280px and 375px, against both the live GitHub API and the mocks, including resizing mid-flow.

## Behaviour changes

Each of these fixes a bug in the Backbone version or comes from a choice made for the port:

- **Live breakpoint.** The original decided phone or desktop once at startup. Now resizing switches layouts, and a phone page that becomes desktop-sized goes home, where the same data shows as panels.
- **URLs for phone pages.** The original never changed the URL. Deep links or refreshes on a phone page with nothing loaded redirect home.
- **No leaked views.** Pressing Get User Activity twice on desktop left the old Backbone view bound to the same element. Now the panel is replaced.
- **Back from the repo table** goes to the categories page directly. The original called `goToRepoCategoryPage()` with no arguments and only worked because that page happened to exist.
- **Usernames are URL-encoded** in API calls.

Kept as-is: the user lookup still uses GitHub's `legacy/user/search` endpoint. It still responds, but
`gravatar_id` is now always empty, so the avatar is Gravatar's default image in both apps.

## Result

- Same screens, text, element ids and API calls as the Backbone app
- Works on desktop and phone widths, and switches live on resize
- 23/23 tests passing, clean TypeScript build, no console warnings

## Projects

| Folder | Stack |
|---|---|
| [`backbone-js/`](./backbone-js) | Backbone 1.0 + RequireJS + jQuery Mobile 1.3 + Handlebars + EnquireJS |
| [`react-js/`](./react-js) | React 18 + Vite + TypeScript + React Router 6 + TanStack Query |

`react-js/` is a standalone project with its own `package.json`, README and tests. `backbone-js/` has
no build step: serve the folder statically and open `index.html`.
