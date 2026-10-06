# Vanilla JS Weather App (React)

React port of [`../vanilla-js`](../vanilla-js): Vite + TypeScript + TanStack Query.

```bash
npm install
cp .env.example .env.local   # then set VITE_OPENWEATHER_API_KEY (free at openweathermap.org)
npm run dev                  # http://localhost:5173, talks to the live OpenWeatherMap API
npm run dev:mock             # canned data, no key needed: try London, Oslo, Atlantis (forecast fails)
npm test                     # Vitest + Testing Library + MSW
npm run build                # typecheck + production build
```

## Vanilla JS → React map
| Vanilla JS | React |
|---|---|
| `index.html` body markup | `src/App.tsx` + `src/components/*` |
| `js/api.js` | `src/api/weather.ts`, `src/api/types.ts`, `src/hooks/queries.ts` |
| Theme code at the top of `js/app.js` | `src/hooks/useTheme.ts`, `src/components/ThemeToggle.tsx`, inline script in `index.html` |
| Search click/Enter handlers | `src/components/SearchBar.tsx`, `CurrentWeather.tsx`, `ErrorBanner.tsx` |
| `displayForecast()` | `src/lib/forecast.ts` (`groupDailyForecast`) + `Forecast.tsx`, `ForecastCard.tsx` |
| Number formatting inline in handlers | `src/lib/format.ts` |
| `css/*.css` | `src/styles/*.css` (unchanged), plus `root.css` |
