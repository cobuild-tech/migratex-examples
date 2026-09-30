import { Navigate, Outlet, useParams } from 'react-router-dom';
import { ArticleForm } from '../components/ArticleForm';
import { useArticle } from '../hooks/queries';
import { useSession } from '../session/SessionContext';
import { NotFoundPage } from './NotFoundPage';
import { articlePath } from '../lib/paths';

export function EditorLayout() {
  return (
    <div className="editor-page">
      <div className="container page">
        <div className="row">
          <div className="col-md-10 offset-md-1 col-xs-12">
            <Outlet />
          </div>
        </div>
      </div>
    </div>
  );
}

export function EditorNewPage() {
  return <ArticleForm />;
}

export function EditorEditPage() {
  const { id = '' } = useParams();
  const { user } = useSession();
  const { data: article, isError } = useArticle(id);

  if (isError) return <NotFoundPage />;
  if (!article) return null;
  if (!user || article.author.username !== user.username) {
    return <Navigate to={articlePath(article.slug)} replace />;
  }
  // key: remount the form (fresh state) when switching between articles
  return <ArticleForm key={article.slug} article={article} />;
}
