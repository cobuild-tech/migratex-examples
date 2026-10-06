import { render, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { App, createQueryClient } from '../../src/App';
import { NOW } from '../mocks/data';

/** Boots the app with "now" frozen at NOW (only Date is faked, so timers and user events work). */
export function visit() {
  vi.useFakeTimers({ toFake: ['Date'] });
  vi.setSystemTime(NOW);
  const user = userEvent.setup();
  const utils = render(<App queryClient={createQueryClient()} />);
  const $ = (selector: string) => document.querySelector<HTMLElement>(selector);
  const $$ = (selector: string) => Array.from(document.querySelectorAll<HTMLElement>(selector));

  async function find(selector: string) {
    await waitFor(() => expect($(selector)).not.toBeNull());
    return $(selector)!;
  }

  return {
    ...utils,
    user,
    $,
    $$,
    find,
    /** Types a city and presses Search (or Enter), then waits for the request to finish. */
    async searchFor(city: string, { enter = false } = {}) {
      await user.clear($('.js-weather-input')!);
      await user.type($('.js-weather-input')!, enter ? `${city}{Enter}` : city);
      if (!enter) await user.click($('.js-weather-search-button')!);
      await waitFor(() => expect($('.js-weather-search-button')).toHaveTextContent('Search'));
    },
    async showForecast() {
      await user.click($('.js-weather-forecast-button')!);
      await waitFor(() => expect($('#forecast-button')).not.toBeDisabled());
    },
    forecastDays: () => $$('.weather__forecast-item h3').map((h3) => h3.textContent),
  };
}
