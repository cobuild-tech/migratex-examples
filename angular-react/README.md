# Migration: Angular → React

The [`react-js/`](./react-js) app is a complete rewrite of [`angular-js/`](./angular-js), built to be
functionally and visually identical to the original — same features, same
styling, same behaviour, same test coverage. This document describes how
that migration was approached.

## Strategy

Given the app's small size and low risk (no production users depending on
it), a **full rewrite** was chosen over an incremental migration. The
existing app was read and understood completely — every component, service,
and style file — before any React code was written.

## Architectural mapping

| Angular concept | React equivalent | Notes |
|---|---|---|
| Template (`.html`) + component class | JSX inside the component function | Markup and logic live together in one file instead of two |
| Two-way binding (`[(ngModel)]`) | Controlled inputs (`value` + `onChange`) | More explicit; every state change is visible in code |
| `@Input()` / `@Output()` | Props / callback props | No decorators needed |
| Injectable service (`LocalStorageService`) | Plain exported functions | No DI container required — the class existed only to group two functions |
| Component state (`AppComponent` fields) | Custom hook (`useTodos`) | All state and handlers extracted into one hook, keeping `App.tsx` focused on rendering |
| `trackBy` in `*ngFor` | React's built-in `key` prop | Same purpose: stable identity per list item across re-renders |
| Angular Router | Not used | Not needed — this app has no routes |
| Angular DI-based state sharing | `useState` only | No Context or external state library — the app is too small to need one |

## Process followed

1. **Read the whole Angular codebase first** — every file, before writing anything new.
2. **Scaffolded the React project** (Vite + React + TypeScript) and got build/dev/test tooling working before writing any feature code.
3. **Migrated bottom-up** — the smallest, most isolated piece (`TodoItem`) first, then the root `App` component that depends on it.
4. **Ported every existing test** (Jasmine/Karma → Vitest + React Testing Library), scenario for scenario, so behavioural coverage was never reduced.
5. **Verified visually** — ran both apps side by side in a real browser (add, prioritize, complete, delete-with-confirm) and compared screenshots, in addition to the automated test suite passing and the production build succeeding.

## Result

- Same `localStorage` keys (`todoList`, `completeList`) and JSON shape, so data format is compatible between both versions
- Same CSS custom properties, layout, and priority colors — no visual regression
- Same confirm-before-delete behaviour
- 13/13 tests passing, clean TypeScript build, no console errors

A longer, first-person write-up of this migration (and general notes on
Angular-to-React modernization strategy for larger apps) exists separately
as a blog post.

## Projects

| Folder | Stack |
|---|---|
| [`angular-js/`](./angular-js) | Angular 14 |
| [`react-js/`](./react-js) | React 19 + Vite + TypeScript |

Each folder is a standalone project with its own `package.json`, README, tests, and MIT license file.
