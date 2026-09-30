import { PER_PAGE } from '../api/endpoints';
import { useArticles, useFeed } from '../hooks/queries';
import { useSession } from '../session/SessionContext';
import { ArticlePreview } from './ArticlePreview';
import { Pagination } from './Pagination';

export function ArticleList({ feed, tag, page }: { feed: string | null; tag: string | null; page: number }) {
  const { user } = useSession();
  // "Your Feed" needs a user; logged-out visitors fall back to the global list.
  const isYourFeed = feed === 'your' && !!user;
  const feedQuery = useFeed(page, isYourFeed);
  const listQuery = useArticles({ tag, page }, !isYourFeed);
  const { data, isLoading } = isYourFeed ? feedQuery : listQuery;

  if (isLoading || !data) {
    return <div className="article-preview">Loading...</div>;
  }

  return (
    <div>
      {data.articles.length ? (
        data.articles.map((article) => <ArticlePreview key={article.slug} article={article} />)
      ) : (
        <div className="article-preview">No articles are here... yet.</div>
      )}
      {data.articlesCount > 0 && (
        <Pagination total={data.articlesCount} perPage={PER_PAGE} current={page} feed={feed} tag={tag} />
      )}
    </div>
  );
}
