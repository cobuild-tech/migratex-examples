import { useSyncExternalStore } from 'react';

/** The breakpoint from `app-phone.css` and the controller's enquire check. */
export const PHONE_QUERY = 'all and (max-width: 599px)';

function subscribe(onChange: () => void) {
  const mql = window.matchMedia(PHONE_QUERY);
  mql.addEventListener('change', onChange);
  return () => mql.removeEventListener('change', onChange);
}

/**
 * Whether views render as separate phone pages or as panels on the home page. Unlike the
 * Backbone controller, which checked once at startup, this follows the window as it resizes.
 */
export function useIsPhone() {
  return useSyncExternalStore(subscribe, () => window.matchMedia(PHONE_QUERY).matches);
}
