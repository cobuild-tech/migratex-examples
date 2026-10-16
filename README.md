# MigrateX Examples

Example migrations from [MigrateX](https://cobuildx.ai) by CobuildX AI. Each folder holds one migration: the original app, its rewrite, and a README describing how the move was made.

| Example | What it shows |
|---|---|
| [`angular-react/`](./angular-react) | An Angular 14 todo app and its React 19 rewrite, with identical features, styling and tests |
| [`ember-react/`](./ember-react) | An Ember 3.24 Octane blogging app (RealWorld API) and its React 18 rewrite, with the same routes, markup and ported acceptance tests |
| [`backbone-react/`](./backbone-react) | A Backbone 1.0 + jQuery Mobile responsive GitHub viewer and its React 18 rewrite, with the same screens and a live desktop/phone layout switch |
| [`vanillajs-react/`](./vanillajs-react) | A plain HTML/CSS/JavaScript weather app (OpenWeatherMap) and its React 18 rewrite, with the same markup, styles and API calls, plus a test suite the original never had |
| [`jquery-react/`](./jquery-react) | A jQuery 3.6 tip calculator and its React 18 rewrite, with the same markup and stylesheet, its calculation bugs fixed, and a test suite the original never had |
| [`cobol-rust/`](./cobol-rust) | A GnuCOBOL banking system (menus, accounts, transfers, cards, on sequential files) and its Rust rewrite, matching the original byte for byte in a compat mode and fixing nine bugs by default |
| [`php-rust/`](./php-rust) | A plain PHP + PDO + MySQL REST API (students CRUD, no framework) and its Rust (axum + sqlx) rewrite behind an nginx route switch, matching PHP byte for byte, notices included, in a compat mode and fixing its bugs by default |
