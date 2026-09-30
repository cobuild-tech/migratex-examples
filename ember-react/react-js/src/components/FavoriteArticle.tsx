import { useNavigate } from 'react-router-dom';
import type { Article } from '../api/types';
import { useFavorite } from '../hooks/queries';
import { useSession } from '../session/SessionContext';

export function FavoriteArticle({
  article,
  isIconOnly = false,
  className = '',
}: {
  article: Article;
  isIconOnly?: boolean;
  className?: string;
}) {
  const { isLoggedIn } = useSession();
  const navigate = useNavigate();
  const favorite = useFavorite();

  const onClick = () => {
    if (isLoggedIn) {
      favorite.mutate(article);
    } else {
      navigate('/login');
    }
  };

  return (
    <button
      className={`btn btn-sm btn${article.favorited ? '' : '-outline'}-primary ${className}`.trim()}
      data-test-favorite-article-button={article.favorited ? 'favorited' : 'unfavorited'}
      onClick={onClick}
      disabled={favorite.isPending}
      type="button"
    >
      <i className="ion-heart"></i>
      &nbsp;
      {isIconOnly ? (
        article.favoritesCount
      ) : (
        <>
          {article.favorited ? 'Unf' : 'F'}avorite Post <span className="counter">{article.favoritesCount}</span>
        </>
      )}
    </button>
  );
}
