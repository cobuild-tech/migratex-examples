import { render, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { createMemoryRouter } from 'react-router-dom';
import { App, createQueryClient } from '../../src/App';
import { routerFuture, routes } from '../../src/routes';
import { KNOWN_USER } from '../mocks/data';
import { setPhone } from './media';

/** Boots the full app at `url` on a phone or desktop viewport. */
export function visit(url = '/', { phone = false } = {}) {
  setPhone(phone);
  const router = createMemoryRouter(routes, { initialEntries: [url], future: routerFuture });
  const user = userEvent.setup();
  const utils = render(<App router={router} queryClient={createQueryClient()} />);
  const $ = (selector: string) => document.querySelector<HTMLElement>(selector);
  const $$ = (selector: string) => Array.from(document.querySelectorAll<HTMLElement>(selector));

  async function find(selector: string) {
    await waitFor(() => expect($(selector)).not.toBeNull());
    return $(selector)!;
  }

  /** Waits until no request is in flight (the loader is gone). */
  async function settled() {
    await waitFor(() => expect($('.ui-loader')).toBeNull());
  }

  return {
    ...utils,
    router,
    user,
    $,
    $$,
    find,
    settled,
    currentURL: () => router.state.location.pathname,
    click: async (selector: string) => user.click(await find(selector)),
    /** Types a username and presses Find User, then waits for the lookup. */
    async findUser(name = KNOWN_USER) {
      await user.clear($('#gh-username')!);
      await user.type($('#gh-username')!, name);
      await user.click($('#find-user')!);
      await settled();
    },
    /** Text of each body row, cells joined with " | ". */
    rows: (scope: string) =>
      $$(`${scope} tbody tr`).map((row) =>
        Array.from(row.children)
          .map((cell) => cell.textContent)
          .join(' | '),
      ),
  };
}
