import { Link } from 'react-router-dom';
import { useTags } from '../hooks/queries';
import { homeUrl } from '../lib/homeUrl';

export function TagList() {
  const { data: tags = [], isLoading } = useTags();

  return (
    <>
      <p>Popular Tags</p>
      {isLoading ? (
        <p>Loading...</p>
      ) : (
        <div className="tag-list">
          {tags.map((tag) => (
            <Link key={tag} to={homeUrl({ tag })} className="tag-pill tag-default" data-test-tag={tag}>
              {tag}
            </Link>
          ))}
        </div>
      )}
    </>
  );
}
