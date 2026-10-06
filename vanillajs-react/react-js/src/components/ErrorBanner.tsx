import { MissingApiKeyError } from '../api/weather';

const FETCH_FAILED = ' Oops! Could not fetch weather data. Please try again later.';

export function ErrorBanner({ error }: { error: Error }) {
  return (
    // `.error` is display: none in the stylesheet; the original showed it with an inline style.
    <div id="error-container" className="error" role="alert" style={{ display: 'block' }}>
      <img className="error-icon" src="/img/error-icon.svg" alt="" />
      <p className="error-info">{error instanceof MissingApiKeyError ? error.message : FETCH_FAILED}</p>
    </div>
  );
}
