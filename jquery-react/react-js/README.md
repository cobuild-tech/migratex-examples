# jQuery Tip Calculator (React)

React port of [`../jquery-js`](../jquery-js): Vite + TypeScript.

```bash
npm install
npm run dev      # http://localhost:5173
npm test         # Vitest + Testing Library
npm run build    # typecheck + production build
```

## jQuery → React map
| jQuery | React |
|---|---|
| `index.html` body markup | `src/App.tsx` + `src/components/*` |
| `total`, `people`, `tipPerc` variables in `scripts.js` | `src/state.ts` (`useReducer`) |
| `calculation()` | `src/lib/split.ts` (`calculateSplit`, `formatMoney`) |
| `.bill-input` / `.people-input` `input` handlers | `BillInput.tsx`, `PeopleInput.tsx`, filters in `src/lib/sanitize.ts` |
| `.share` click + `.shareinput` change handlers | `TipSelector.tsx` |
| `.result-btn-div` click handler, `resetActive()` | `ResultsPanel.tsx` + the `reset` action |
| `styles.css`, `images/` | `src/styles/styles.css` (two small edits), `src/images/`, plus `root.css` |
