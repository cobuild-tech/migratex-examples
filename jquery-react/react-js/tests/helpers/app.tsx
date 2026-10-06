import { render } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { App } from '../../src/App';

/** Renders the calculator and returns helpers that use the original jQuery selectors. */
export function visit() {
  const user = userEvent.setup();
  const utils = render(<App />);
  const $ = (selector: string) => document.querySelector<HTMLElement>(selector)!;
  const $$ = (selector: string) => Array.from(document.querySelectorAll<HTMLElement>(selector));

  async function fill(selector: string, text: string) {
    await user.clear($(selector));
    if (text) await user.type($(selector), text);
  }

  return {
    ...utils,
    user,
    $,
    $$,
    fill,
    bill: (text: string) => fill('.bill-input', text),
    people: (text: string) => fill('.people-input', text),
    customTip: (text: string) => fill('.shareinput', text),
    preset: (label: string) => user.click(utils.getByRole('button', { name: label })),
    reset: () => user.click(utils.getByRole('button', { name: 'RESET' })),
    tip: () => $('#tip-display').textContent,
    total: () => $('#total-display').textContent,
    selected: () => $$('.tip-selected').map((el) => el.textContent || (el as HTMLInputElement).placeholder),
  };
}
