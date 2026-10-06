# Migration: Vanilla JS → React

The [`react-js/`](./react-js) app is a complete rewrite of [`vanilla-js/`](./vanilla-js), a weather app
built with plain HTML, BEM CSS and ES module JavaScript on the OpenWeatherMap API. You search for a
city to see its current weather, open a 5-day forecast, and switch between a light and a dark theme
that is remembered. The rewrite keeps the same markup, class names, text, stylesheets and API calls.
This document describes how that migration was approached.

## Strategy

The app is small (one page, two API calls, about 230 lines of JavaScript), so a **full rewrite**
was chosen over an incremental migration. The original had no tests, so its behaviour was recorded
from the code first and then written down as the React test suite.

The original wrote all of its markup into `index.html` up front and changed it in place: it looked up
elements, set `textContent` and `src`, toggled `.is-hidden` and `style.display`, and built forecast
cards one `createElement` at a time. In React the same markup is rendered from state, so most of
that DOM work disappears, and what is left is state, data fetching and one pure function.

## Architectural mapping

| Vanilla JS | React equivalent | Notes |
|---|---|---|
| Markup in `index.html` + `querySelector` refs | JSX components with the same class names, ids and text | `js-*` hook classes kept so the DOM matches |
| `.is-hidden` / `style.display` toggling | Conditional rendering | The Show button still toggles `.is-hidden`, as the CSS expects |
| Values read from the input at click time | Controlled input; the city is committed as a `search` on submit | The forecast belongs to the searched city |
| `keypress` Enter → `searchButton.click()` | `onKeyDown` Enter on the input | Same markup, no `<form>` added |
| `fetch` inside click handlers | `fetch` functions + TanStack Query hooks | Each search is a new request, as before |
| `displayForecast()` building elements | `groupDailyForecast()` + `<ForecastCard>` | The grouping is a pure, unit-tested function |
| Theme applied by the deferred module | `useTheme()` + an inline script in `<head>` | The saved theme is applied before first paint |
| API key hard-coded in `api.js` | `VITE_OPENWEATHER_API_KEY` in `.env.local` | A missing key shows a clear message |
| `css/*.css` | Same files under `src/styles/` | Plus `#root { display: contents }` so the body's centred flex layout still applies |

## Process followed

1. **Read the whole app** and listed every behaviour, including the odd ones: the forecast reads the input box rather than the searched city, the "Loading..." text is set on a hidden section and never seen, and a day with no midday entry gets an empty icon URL.
2. **Scaffolded the React project** (Vite + React + TypeScript), matching the stack of the other examples.
3. **Built the foundations**: the API functions and types, query hooks, the theme hook, the formatting helpers and the forecast grouping.
4. **Migrated the screen piece by piece**: theme toggle, search, current weather, error banner, forecast.
5. **Kept the stylesheets unchanged**, with one wrapper rule so React's root element doesn't break the layout.
6. **Wrote the tests** against MSW mocks of the two OpenWeatherMap endpoints, with "now" frozen so the forecast days are fixed.
7. **Verified in a real browser** at 1280px and 375px, in both themes, side by side with the original against the live API, and against the mocks.

## Behaviour changes

Each of these fixes a bug in the vanilla JS version or comes from a choice made for the port:

- **The forecast is for the searched city.** The original read the input box when Show was clicked, so editing the input first gave a forecast for a different city than the one on screen.
- **A new search closes the old forecast.** The original left the previous city's forecast open under the new result.
- **Forecast errors are shown.** The original only logged them to the console; now the error banner appears and Show can be pressed again.
- **No broken forecast icons.** A day with no 10:00-14:00 entry (usually the last, partial day) got the URL `wn/@2x.png`. It now uses the entry closest to noon.
- **A visible loading state.** The original set "Loading..." on a section it had just hidden. Now the Search button reads "Loading..." and is disabled while the request runs.
- **No theme flash.** The saved theme is applied before first paint instead of after the module loads.
- **The API key comes from the environment** instead of the source, and the error icon has an empty `alt` because it is decorative.

Kept as-is: current temperature rounded down to one decimal and forecast temperatures rounded to whole
degrees, today left out of the forecast, the same OpenWeatherMap query parameters, the same error text.
Hiding and re-showing the forecast reuses the data already fetched for that search, where the original
fetched it again.

## Result

- Same markup, class names, text, stylesheets and API calls as the vanilla JS app
- Works in both themes at desktop and phone widths
- 24/24 tests passing, clean TypeScript build, no console warnings

## Projects

| Folder | Stack |
|---|---|
| [`vanilla-js/`](./vanilla-js) | HTML5 + BEM CSS + ES module JavaScript + Fetch API |
| [`react-js/`](./react-js) | React 18 + Vite + TypeScript + TanStack Query |

`react-js/` is a standalone project with its own `package.json`, README and tests. `vanilla-js/` has
no build step: set an OpenWeatherMap key in `js/api.js`, serve the folder statically and open `index.html`.
