import { Link } from 'react-router-dom';
import { homeUrl } from '../lib/homeUrl';

export function Pagination({
  total,
  perPage,
  current,
  feed,
  tag,
}: {
  total: number;
  perPage: number;
  current: number;
  feed?: string | null;
  tag?: string | null;
}) {
  const count = total ? Math.ceil(total / perPage) : 0;
  const pages = Array.from({ length: count }, (_, i) => i + 1);

  return (
    <nav>
      <ul className="pagination">
        {pages.map((page) => (
          <li key={page} className={`page-item ${page === current ? 'active' : ''}`} data-test-page-item={page}>
            <Link to={homeUrl({ feed, tag, page })} className="page-link" data-test-page-item-link={page}>
              {page}
            </Link>
          </li>
        ))}
      </ul>
    </nav>
  );
}
