import type { ReactNode } from 'react';
import { useLocation } from 'react-router-dom';

export interface PageState {
  /** Slide in from the left, as jQuery Mobile's `reverse: true` did for going back. */
  reverse?: boolean;
  /** Skip the slide (the first page, and redirects). */
  transition?: 'none';
}

/** A full-screen page (`data-role="page"`), sliding in when navigated to. */
export function Page({ id, children }: { id: string; children: ReactNode }) {
  const location = useLocation();
  const state = (location.state ?? {}) as PageState;
  const animate = location.key !== 'default' && state.transition !== 'none';
  const transition = animate ? (state.reverse ? ' slide-in reverse' : ' slide-in') : '';
  return (
    <div id={id} className={`ui-page${transition}`} key={location.key}>
      {children}
    </div>
  );
}
