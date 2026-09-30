import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { createMemoryRouter } from 'react-router-dom';
import { App, createQueryClient } from '../../src/App';
import { routerFuture, routes } from '../../src/routes';
import { STORAGE_KEY } from '../../src/session/SessionContext';

export const TOKEN = 'auth-token';

export function logInAs(token = TOKEN) {
  localStorage.setItem(STORAGE_KEY, token);
}

/** Boots the full app at `url` (the equivalent of Ember's `visit()`). */
export async function visit(url: string) {
  const router = createMemoryRouter(routes, { initialEntries: [url], future: routerFuture });
  const user = userEvent.setup();
  const utils = render(<App router={router} queryClient={createQueryClient({ retry: false })} />);
  // The session gate renders nothing until the stored token is resolved.
  await waitFor(() => expect(utils.container.querySelector('nav.navbar')).not.toBeNull());
  return {
    ...utils,
    router,
    user,
    currentURL: () => router.state.location.pathname + router.state.location.search,
    $: (selector: string) => utils.container.querySelector<HTMLElement>(selector),
    $$: (selector: string) => Array.from(utils.container.querySelectorAll<HTMLElement>(selector)),
    /** Waits for an element matching `selector` and returns it. */
    find: async (selector: string) => {
      let el: HTMLElement | null = null;
      await waitFor(() => {
        el = utils.container.querySelector<HTMLElement>(selector);
        expect(el).not.toBeNull();
      });
      return el as unknown as HTMLElement;
    },
  };
}

export type AppHarness = Awaited<ReturnType<typeof visit>>;
export { screen, waitFor };
