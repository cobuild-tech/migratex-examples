# Inkwell (React)

React port of [`../ember-js`](../ember-js): Vite + TypeScript + React Router 6 (data router) + TanStack Query.

```bash
npm install
npm run dev        # http://localhost:5173, talks to VITE_API_HOST (.env)
npm test           # Vitest + Testing Library + MSW (ported Ember acceptance tests)
npm run build      # typecheck + production build
```

## Ember → React map
| Ember | React |
|---|---|
| `services/session.js` | `src/session/SessionContext.tsx` (same localStorage key) |
| Ember Data models/adapters/serializers | `src/api/*` + `src/hooks/queries.ts`; identity-map syncing in `src/lib/cacheUpdates.ts` |
| `router.js` + `routes/*` | `src/routes/index.tsx`, `RequireAuth.tsx` (was `logged-in.js`) |
| `templates/*` | `src/pages/*` |
| `components/*` | `src/components/*` (same CSS classes and `data-test-*` attributes) |
| `helpers/format-date.js` | `src/lib/formatDate.ts` |
| Mirage | `tests/mocks/*` (MSW) |
