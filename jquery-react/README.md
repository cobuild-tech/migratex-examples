# Migration: jQuery → React

The [`react-js/`](./react-js) app is a complete rewrite of [`jquery-js/`](./jquery-js), a tip calculator
built with static HTML, plain CSS and jQuery 3.6. You enter a bill, pick a preset tip (5, 10, 15, 25 or
50%) or type a custom one, enter the number of people, and see the tip and the total per person. The
rewrite keeps the same markup, class names, text and stylesheet. This document describes how that
migration was approached.

## Strategy

The app is tiny (one screen, about 70 lines of jQuery), so a **full rewrite** was chosen over an
incremental migration. The original had no tests, so its behaviour was recorded from the code first and
then written down as the React test suite.

The jQuery code kept three loose variables (`total`, `people`, `tipPerc`) and copied results into the
page with `.html()`, toggling classes and the Reset button by hand in each event handler. Every handler
had to keep those variables, the inputs and the screen in step, and several didn't. In React the inputs
are the state and the results are worked out from it on every render, so they can't drift apart.

## Architectural mapping

| jQuery | React equivalent | Notes |
|---|---|---|
| `$(document).ready` + markup in `index.html` | JSX components with the same class names, ids and text | `#tip-display`, `#total-display` and every class kept |
| Module-level `total` / `people` / `tipPerc` | `useReducer` over the raw input strings | One `reset` action returns the initial state |
| `calculation()` writing with `.html()` | `calculateSplit()` called during render | Pure and unit-tested |
| `.addClass('tip-selected').siblings().removeClass(...)` | `className` from the selected tip in state | Presets also get `aria-pressed` |
| `.invalid` `.show()` / `.hide()`, `invalid-people` class | Inline `display` and `className` from state | Inline style, like `.show()`, so the mobile `.bill span { display: none }` rule doesn't hide it |
| `.prop('disabled', ...)` in `resetActive()` | `disabled` worked out from state | Enabled while any field has a value or a tip is chosen |
| `on('propertychange input')` | Controlled inputs with input filters | Fields stay `type="text"`, as the CSS expects, with `inputMode` for phone keyboards |
| jQuery from the CDN | Removed | No runtime dependencies besides React |
| `styles.css`, `images/` | `src/styles/styles.css`, `src/images/` | Plus `#root { display: contents }` so the body layout is unchanged |

## Process followed

1. **Read the whole app** and listed every behaviour, including the broken ones: the custom tip is never used, Reset leaves the old tip in memory, and some input orders print `NaN` or `Infinity`.
2. **Scaffolded the React project** (Vite + React + TypeScript), matching the stack of the other examples.
3. **Wrote the logic first**: the split calculation, the input filters and the reducer, each with unit tests.
4. **Migrated the markup** into four components (bill, tip selector, people, results) using the original class names.
5. **Copied the stylesheet**, changing only the icon paths and the tip button rule (see below).
6. **Wrote acceptance tests** that drive the app through the original jQuery selectors.
7. **Verified in a real browser** side by side with the original. At 1280px the two screenshots are pixel-identical. At 375px they differ in 77 anti-aliased pixels inside one button's "5%" label, with identical layout and computed styles.

## Behaviour changes

Each of these fixes a bug in the jQuery version or comes from a choice made for the port:

- **The custom tip works.** The original only highlighted the Custom field; the value was never used in the calculation.
- **Reset clears the state, not just the screen.** The original left the old tip and people count in memory, so after a reset a new bill used the previous tip while no button was highlighted.
- **Reset only responds to the button.** The handler was on the wrapping `div`, so clicking around a disabled Reset button still cleared the form.
- **No `NaN` or `Infinity`.** Entering people before a bill, clearing the people field, or changing the bill while people was 0 showed `NaN` or `Infinity`. The results now read `0.00` until both fields hold valid values.
- **Results go back to `0.00` when people is 0.** The original left the previous results on screen next to "Can't be zero". Changing the bill no longer hides the error either.
- **Rounding happens once.** The original rounded the base and the tip per person separately, so the total could be a cent off (a 10.00 bill at 10% for 3 people showed 3.66 instead of 3.67).
- **Always two decimals.** The tip used `parseFloat` and could show `4.3` or `5`.
- **Only numbers can be typed.** Letters, minus signs and a second decimal point are ignored. The bill takes at most two decimals, and people takes whole numbers only.
- **Valid, accessible markup.** Labels point at real inputs, the stray `</input>` tags are gone, tip buttons are `type="button"` with `aria-pressed`, and the logo has `alt="Splitter"`.
- **Tip button text is a `<span>`, not an `<h3>`.** Headings aren't allowed inside buttons, so `.share h3` became `.share span` with `display: block; font-weight: 700` to look the same.

Kept as-is: the layout, colours, fonts and 500px breakpoint, every label and the "Can't be zero" text,
the five preset values, and Reset starting out disabled. The CSS is unchanged in one more place: a
focused people field shows the teal focus outline instead of the red error outline, because
`.people-input:focus` comes after `.invalid-people:focus` in the original stylesheet.

## Result

- Same markup, class names, text and stylesheet as the jQuery app
- Pixel-identical on desktop, and the same layout on phones
- 19/19 tests passing, clean TypeScript build, no console errors

## Projects

| Folder | Stack |
|---|---|
| [`jquery-js/`](./jquery-js) | HTML5 + CSS3 + jQuery 3.6 |
| [`react-js/`](./react-js) | React 18 + Vite + TypeScript |

`react-js/` is a standalone project with its own `package.json`, README and tests. `jquery-js/` has no
build step: open `index.html` in a browser (it loads jQuery from the CDN).
