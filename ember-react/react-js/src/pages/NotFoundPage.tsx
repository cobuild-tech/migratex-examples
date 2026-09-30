import { Link } from 'react-router-dom';

export function NotFoundPage() {
  return (
    <div className="editor-page" data-test-error-page>
      <div className="container page">
        <div className="row">
          <div className="col-md-10 offset-md-1 col-xs-12">
            <h2>Sorry but that page does not exist.</h2>
            <p>
              Try find your page from the <Link to="/">homepage</Link>
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}
