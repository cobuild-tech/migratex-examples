import { Link } from 'react-router-dom';
import type { Article } from '../api/types';
import { useSession } from '../session/SessionContext';
import { ArticleAuthor } from './ArticleAuthor';
import { FavoriteArticle } from './FavoriteArticle';
import { FollowProfile } from './FollowProfile';
import { editorPath } from '../lib/paths';

export function ArticleMeta({
  article,
  onDelete,
  isDeleting,
}: {
  article: Article;
  onDelete(): void;
  isDeleting: boolean;
}) {
  const { user } = useSession();

  return (
    <div className="article-meta">
      <ArticleAuthor author={article.author} updatedAt={article.updatedAt} />
      {user && user.username === article.author.username ? (
        <>
          <Link className="btn btn-outline-secondary btn-sm" to={editorPath(article.slug)} data-test-edit-article-button>
            <i className="ion-edit"></i> Edit Article
          </Link>{' '}
          <button
            className="btn btn-outline-danger btn-sm"
            onClick={onDelete}
            disabled={isDeleting}
            type="button"
            data-test-delete-article-button
          >
            <i className="ion-trash-a"></i> Delete Article
          </button>
        </>
      ) : (
        <>
          <FollowProfile profile={article.author} />
          &nbsp;&nbsp;
          <FavoriteArticle article={article} />
        </>
      )}
    </div>
  );
}
