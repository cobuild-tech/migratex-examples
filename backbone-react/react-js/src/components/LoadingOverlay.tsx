import { useIsFetching, useIsMutating } from '@tanstack/react-query';
import { useEffect } from 'react';

/** The `$.ajaxSetup` loader: disables the page and shows "Connecting To Github" during requests. */
export function LoadingOverlay() {
  const busy = useIsFetching() + useIsMutating() > 0;

  useEffect(() => {
    document.body.classList.toggle('ui-disabled', busy);
  }, [busy]);

  if (!busy) return null;
  return (
    <div className="ui-loader" role="status">
      <span className="ui-loader-icon" aria-hidden="true" />
      <h1>Connecting To Github</h1>
    </div>
  );
}
