import { Link } from 'react-router-dom';
import type { Profile } from '../api/types';
import { formatDate } from '../lib/formatDate';
import { profilePath } from '../lib/paths';

export function ArticleAuthor({ author, updatedAt }: { author: Profile; updatedAt: string }) {
  return (
    <>
      <Link to={profilePath(author.username)}>
        <img src={author.image ?? undefined} alt={author.username} />
      </Link>
      <div className="info">
        <Link to={profilePath(author.username)} className="author">
          {author.username}
        </Link>
        <span className="date">{formatDate(updatedAt)}</span>
      </div>
    </>
  );
}
