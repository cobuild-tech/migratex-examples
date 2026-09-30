import { Link } from 'react-router-dom';

export function Footer() {
  return (
    <footer>
      <div className="container">
        <Link to="/" className="logo-font">
          inkwell
        </Link>
        <span className="attribution">A place to share your knowledge.</span>
      </div>
    </footer>
  );
}
