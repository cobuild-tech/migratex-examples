import { act } from '@testing-library/react';

let phone = false;
const listeners = new Set<() => void>();

/** A controllable `window.matchMedia`; only the phone breakpoint query is used by the app. */
export function installMatchMedia() {
  window.matchMedia = ((query: string) => ({
    get matches() {
      return phone;
    },
    media: query,
    onchange: null,
    addEventListener: (_: string, listener: () => void) => listeners.add(listener),
    removeEventListener: (_: string, listener: () => void) => listeners.delete(listener),
    addListener: () => {},
    removeListener: () => {},
    dispatchEvent: () => false,
  })) as unknown as typeof window.matchMedia;
}

/** Sets the viewport to phone or desktop before the app boots. */
export function setPhone(value: boolean) {
  phone = value;
}

/** Simulates resizing across the breakpoint while the app is running. */
export function resizeTo(value: 'phone' | 'desktop') {
  act(() => {
    phone = value === 'phone';
    listeners.forEach((listener) => listener());
  });
}

export function resetMedia() {
  phone = false;
  listeners.clear();
}
