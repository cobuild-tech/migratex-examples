import { useEffect } from 'react';
import { Link, NavLink, useLocation, useNavigate } from 'react-router-dom';
import { useSession } from '../session/SessionContext';
import { profilePath } from '../lib/paths';

export function Nav() {
  const session = useSession();
  const navigate = useNavigate();
  const location = useLocation();

  // Log out only once we've arrived home: leaving first stops auth-only pages (e.g. /settings)
  // redirecting to /login, and a navigation cancelled by the unsaved-changes prompt never logs out.
  const loggingOut = (location.state as { logOut?: boolean } | null)?.logOut;
  useEffect(() => {
    if (loggingOut) {
      session.logOut();
      navigate('/', { replace: true, state: null });
    }
  }, [loggingOut, session, navigate]);

  const logOut = (e: React.MouseEvent) => {
    e.preventDefault();
    navigate('/', { state: { logOut: true } });
  };

  return (
    <nav className="navbar navbar-light">
      <div className="container">
        <Link to="/" className="navbar-brand">
          inkwell
        </Link>
        <ul className="nav navbar-nav pull-xs-right">
          <li className="nav-item">
            <NavLink to="/" end className="nav-link" data-test-nav-home>
              Home
            </NavLink>
          </li>
          {session.isLoggedIn ? (
            <>
              <li className="nav-item">
                <NavLink to="/editor" end className="nav-link" data-test-nav-new-post>
                  <i className="ion-compose"></i>&nbsp;New Post
                </NavLink>
              </li>
              <li className="nav-item">
                <NavLink to="/settings" className="nav-link" data-test-nav-settings>
                  <i className="ion-compose"></i>&nbsp;Settings
                </NavLink>
              </li>
              <li className="nav-item">
                <NavLink to={profilePath(session.user?.username ?? '')} className="nav-link" data-test-nav-username>
                  {session.user?.username}
                </NavLink>
              </li>
              <li className="nav-item">
                <a href="" className="nav-link" onClick={logOut} data-test-nav-log-out>
                  Log out
                </a>
              </li>
            </>
          ) : (
            <>
              <li className="nav-item">
                <NavLink to="/login" className="nav-link" data-test-nav-sign-in>
                  Sign in
                </NavLink>
              </li>
              <li className="nav-item">
                <NavLink to="/register" className="nav-link" data-test-nav-sign-up>
                  Sign up
                </NavLink>
              </li>
            </>
          )}
        </ul>
      </div>
    </nav>
  );
}
